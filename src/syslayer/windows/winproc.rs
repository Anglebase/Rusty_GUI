use std::{os::raw::c_void, sync::Mutex};

use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::winuser::*,
};

use crate::{pos, Canvas, Ele, Event, ModifierKey, MouseButton, MouseWheel};

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
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN | WM_LBUTTONUP
        | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP | WM_LBUTTONDBLCLK | WM_RBUTTONDBLCLK
        | WM_MBUTTONDBLCLK | WM_XBUTTONDBLCLK => {
            handle_mouse_event(obj, msg, wparam, lparam);
        }
        WM_MOUSEMOVE => {
            let pos = lparam_to_pos!(lparam);
            let mk = wparam_to_mkey!(wparam & 0xffff);
            let event = Event::MouseMoved { pos, mk };
            obj.on_event(&event);
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
