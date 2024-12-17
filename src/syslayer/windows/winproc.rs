use std::{os::raw::c_void, sync::Mutex};

use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::winuser::*,
};

use crate::*;

use super::{get_rect, notifier_exit};

static mut WINDOW_COUNT: Mutex<u32> = Mutex::new(0);

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
        let x = $lparam as i32 & 0xffff;
        let y = ($lparam >> 16) as i32 & 0xffff;
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
        // call init
        let object_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Box<dyn Ele>;
        let obj = object_ptr.as_mut().unwrap();
        obj.on_event(&Event::WindowCreated);
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    // get window object
    let object_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Box<dyn Ele>;
    let obj = if object_ptr.is_null() {
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
            let mut hflags = HotKeyFlags::default();
            hflags.alt = (mod_key & MOD_ALT) != 0;
            hflags.ctrl = (mod_key & MOD_CONTROL) != 0;
            hflags.shift = (mod_key & MOD_SHIFT) != 0;
            hflags.win = (mod_key & MOD_WIN) != 0;
            let event = Event::HotKey {
                key: vk_to_key(vk as _),
                modifiers: hflags,
            };
            obj.on_event(&event);
            return 0;
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
