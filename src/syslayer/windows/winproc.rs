use std::{os::raw::c_void, sync::Mutex};

use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::winuser::*,
};

use crate::{pos, Canvas, Ele, Event, MouseButton};

use super::{get_rect, notifier_exit};

static mut WINDOW_COUNT: Mutex<u32> = Mutex::new(0);

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
        WM_LBUTTONDOWN => {
            let x = (lparam & 0xffff) as i32;
            let y = (lparam >> 16) as i32;
            obj.on_event(&Event::MouseButtonPressed {
                button: MouseButton::Left,
                pos: pos!(x, y),
            });
        }
        WM_LBUTTONUP => {
            let x = (lparam & 0xffff) as i32;
            let y = (lparam >> 16) as i32;
            obj.on_event(&Event::MouseButtonReleased {
                button: MouseButton::Left,
                pos: pos!(x, y),
            });
        }
        _ => {}
    };
    DefWindowProcW(hwnd, msg, wparam, lparam)
}
