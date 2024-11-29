use winapi::{shared::windef::*, um::winuser::*};

#[allow(unused)]
pub struct App {}

impl App {
    #[allow(unused)]
    pub fn run() {
        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        unsafe {
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
    #[allow(unused)]
    pub fn exit() {
        unsafe { PostQuitMessage(0) };
    }
}
