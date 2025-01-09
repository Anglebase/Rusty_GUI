use std::any::Any;
use std::{os::raw::c_void, sync::Mutex};
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::{
        wingdi::{SetBkMode, TRANSPARENT},
        winuser::*,
    },
};

use crate::*;

use super::{get_rect, notifier_exit};

static mut WINDOW_COUNT: Mutex<u32> = Mutex::new(0);
pub const USER_DEF_MSG: UINT = WM_USER + 1; //
pub const WINDOW_CREATED_MSG: UINT = WM_USER + 2;

macro_rules! wparam_to_mkey {
    ($wparam:expr) => {
        match $wparam {
            MK_LBUTTON => ModifierKey::Mouse(MouseButton::Left),
            MK_RBUTTON => ModifierKey::Mouse(MouseButton::Right),
            MK_MBUTTON => ModifierKey::Mouse(MouseButton::Middle),
            MK_XBUTTON1 => ModifierKey::Mouse(MouseButton::Other(0x01)),
            MK_XBUTTON2 => ModifierKey::Mouse(MouseButton::Other(0x02)),
            MK_CONTROL => ModifierKey::Ctrl,
            MK_SHIFT => ModifierKey::Shift,
            _ => ModifierKey::None,
        }
    };
}

macro_rules! lparam_to_pos {
    ($lparam:expr) => {{
        let x = ($lparam & 0xffff) as i32;
        let y = (($lparam >> 16) & 0xffff) as i32;
        pos!(x, y)
    }};
}

macro_rules! lparam_to_size {
    ($lparam:expr) => {{
        let width = $lparam as i32 & 0xffff;
        let height = ($lparam >> 16) as i32 & 0xffff;
        size!(width, height)
    }};
}

pub(super) unsafe extern "system" fn winproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // init window data
    if msg == WM_CREATE {
        let create_struct_ptr = lparam as *const CREATESTRUCTW;
        let create_struct = &*create_struct_ptr;
        let object_ptr = create_struct.lpCreateParams as *mut c_void;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, object_ptr as _);
        {
            let mut window_count = WINDOW_COUNT.lock().unwrap();
            *window_count += 1;
        }
        let mut ent = TRACKMOUSEEVENT {
            cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
            dwFlags: TME_HOVER | TME_LEAVE,
            hwndTrack: hwnd,
            dwHoverTime: 0,
        };
        TrackMouseEvent(&mut ent);
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    // get window object
    let object_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut (Box<dyn Ele>, bool);
    let (obj, hover) = if object_ptr.is_null() {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    } else {
        object_ptr.as_mut().unwrap()
    };
    match msg {
        WM_DESTROY => {
            obj.on_event(&Event::WindowDestroyed);
            {
                let mut window_count = WINDOW_COUNT.lock().unwrap();
                *window_count -= 1;
                if *window_count == 0 {
                    notifier_exit(0);
                }
            }
            return 0;
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT {
                hdc: 0 as HDC,
                fErase: 0,
                rcPaint: RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
                fRestore: 0,
                fIncUpdate: 0,
                rgbReserved: [0; 32],
            };
            let hdc = BeginPaint(hwnd, &mut ps);
            SetBkMode(hdc, TRANSPARENT as _);
            let mut canvas = Canvas {
                hdc: hdc as _,
                rect: get_rect(hwnd as _),
            };
            obj.draw(&mut canvas);
            EndPaint(hwnd, &ps);
            return 0;
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN | WM_LBUTTONUP
        | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP | WM_LBUTTONDBLCLK | WM_RBUTTONDBLCLK
        | WM_MBUTTONDBLCLK | WM_XBUTTONDBLCLK => {
            handle_mouse_event(obj, msg, wparam, lparam);
            return 0;
        }
        WM_MOUSEMOVE => {
            let pos = lparam_to_pos!(lparam);
            let mk = wparam_to_mkey!(wparam & 0xffff);
            let event = Event::MouseMoved { pos, mk };
            obj.on_event(&event);
            let mut tme = TRACKMOUSEEVENT {
                cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_HOVER | TME_LEAVE,
                hwndTrack: hwnd,
                dwHoverTime: 1,
            };
            TrackMouseEvent(&mut tme);
            return 0;
        }
        WM_MOUSEWHEEL => {
            let pos = lparam_to_pos!(lparam);
            let mk = wparam_to_mkey!(wparam & 0xffff);
            let delta = (wparam >> 16) as i16;
            let event = Event::MouseWheelScrolled {
                wheel: if delta > 0 {
                    MouseWheel::Up
                } else {
                    MouseWheel::Down
                },
                pos,
                mk,
            };
            obj.on_event(&event);
            return 0;
        }
        WM_SIZE => {
            let size = lparam_to_size!(lparam);
            let ty = match wparam {
                SIZE_MAXIMIZED => WindowSize::Maximize,
                SIZE_MINIMIZED => WindowSize::Minimize,
                SIZE_RESTORED => WindowSize::Restore,
                SIZE_MAXHIDE => WindowSize::MaxHide,
                SIZE_MAXSHOW => WindowSize::MaxShow,
                _ => WindowSize::Resize,
            };
            let event = Event::WindowResized { size, ty };
            obj.on_event(&event);
            return 0;
        }
        WM_MOVE => {
            let pos = lparam_to_pos!(lparam);
            let event = Event::WindowMoved { pos };
            obj.on_event(&event);
            return 0;
        }
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
            handle_key_event(obj, msg, wparam);
            return 0;
        }
        WM_CHAR => {
            let ch: char = std::char::from_u32(wparam as u32).unwrap();
            let event = Event::Input { ch };
            obj.on_event(&event);
            return 0;
        }
        WM_HOTKEY => {
            let mod_key = lparam & 0xffff;
            let vk = (lparam >> 16) & 0xffff;
            let mut flags = HotKeyFlags::default();
            flags.alt = (mod_key & MOD_ALT) != 0;
            flags.ctrl = (mod_key & MOD_CONTROL) != 0;
            flags.shift = (mod_key & MOD_SHIFT) != 0;
            flags.win = (mod_key & MOD_WIN) != 0;
            let event = Event::HotKey {
                key: vk_to_key(vk as _),
                modifiers: flags,
            };
            obj.on_event(&event);
            return 0;
        }
        WM_ENABLE => {
            let enable = wparam != 0;
            let event = if enable {
                Event::WindowEnable
            } else {
                Event::WindowDisable
            };
            obj.on_event(&event);
            return 0;
        }
        WM_TIMER => {
            let id = wparam as usize;
            let event = Event::Timer { id };
            obj.on_event(&event);
            return 0;
        }
        WM_MOUSELEAVE => {
            if *hover {
                let event = Event::Leave;
                obj.on_event(&event);
                *hover = false;
            }
            return 0;
        }
        WM_MOUSEHOVER => {
            if !*hover {
                let pos = lparam_to_pos!(lparam);
                let mk = wparam_to_mkey!(wparam);
                let event = Event::Hover { pos, mk };
                obj.on_event(&event);
                *hover = true;
            }
            return 0;
        }
        WM_GETMINMAXINFO => {
            let minmaxinfo = (lparam as *mut MINMAXINFO).as_mut().unwrap();
            let min_width = obj.as_window().min_width.unwrap_or(0);
            let min_height = obj.as_window().min_height.unwrap_or(0);
            let screen_x = GetSystemMetrics(SM_CXSCREEN);
            let screen_y = GetSystemMetrics(SM_CYSCREEN);
            let max_width = obj.as_window().max_width.unwrap_or(screen_x);
            let max_height = obj.as_window().max_height.unwrap_or(screen_y);
            minmaxinfo.ptMinTrackSize.x = min_width;
            minmaxinfo.ptMinTrackSize.y = min_height;
            minmaxinfo.ptMaxTrackSize.x = max_width;
            minmaxinfo.ptMaxTrackSize.y = max_height;
            minmaxinfo.ptMaxSize.x = max_width;
            minmaxinfo.ptMaxSize.y = max_height;
            minmaxinfo.ptMaxPosition.x = (screen_x - max_width) / 2;
            minmaxinfo.ptMaxPosition.y = (screen_y - max_height) / 2;
            return 0;
        }
        USER_DEF_MSG => {
            let any_obj_ptr = Box::from_raw(lparam as *mut Box<dyn Any>);
            obj.on_message(*any_obj_ptr);
        }
        WINDOW_CREATED_MSG => {
            // call init
            let object_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut (Box<dyn Ele>, bool);
            let (obj, _) = object_ptr.as_mut().unwrap();
            obj.on_event(&Event::WindowCreated);
        }
        _ => {}
    };
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

unsafe fn handle_mouse_event(obj: &mut Box<dyn Ele>, msg: UINT, wparam: WPARAM, lparam: LPARAM) {
    let button = match msg {
        WM_LBUTTONDOWN | WM_LBUTTONUP | WM_LBUTTONDBLCLK => MouseButton::Left,
        WM_RBUTTONDOWN | WM_RBUTTONUP | WM_RBUTTONDBLCLK => MouseButton::Right,
        WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MBUTTONDBLCLK => MouseButton::Middle,
        WM_XBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONUP => {
            if (wparam >> 16) & 0xffff == XBUTTON1.into() {
                MouseButton::Other(0x01)
            } else if (wparam >> 16) & 0xffff == XBUTTON2.into() {
                MouseButton::Other(0x02)
            } else {
                MouseButton::Other(0x00)
            }
        }
        _ => return,
    };
    let pos = lparam_to_pos!(lparam);
    let mk = wparam_to_mkey!(wparam);
    let event = match msg {
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
            Event::MouseButtonPressed { button, pos, mk }
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
            Event::MouseButtonReleased { button, pos, mk }
        }
        WM_LBUTTONDBLCLK | WM_RBUTTONDBLCLK | WM_MBUTTONDBLCLK | WM_XBUTTONDBLCLK => {
            Event::MouseDoubleClicked { button, pos, mk }
        }
        _ => return,
    };
    obj.on_event(&event);
}

unsafe fn handle_key_event(obj: &mut Box<dyn Ele>, msg: UINT, wparam: WPARAM) {
    let vk = wparam as i32;
    let key = vk_to_key(vk);
    let event = match msg {
        WM_KEYDOWN => Event::KeyPressed { key, sys: false },
        WM_KEYUP => Event::KeyReleased { key, sys: false },
        WM_SYSKEYDOWN => Event::KeyPressed { key, sys: true },
        WM_SYSKEYUP => Event::KeyReleased { key, sys: true },
        _ => return,
    };
    obj.on_event(&event);
}

pub unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut (Box<dyn Ele>, bool);
    let obj = if ptr.is_null() {
        return 0;
    } else {
        ptr.as_mut().unwrap().0.as_mut()
    };
    let callback = lparam as *mut Box<dyn FnMut(&mut dyn Ele)>;
    if callback.is_null() {
        return 0;
    }
    callback.as_mut().unwrap()(obj);
    1
}
