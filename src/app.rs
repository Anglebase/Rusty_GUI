use std::ptr::null_mut;

use winapi::{shared::windef::POINT, um::winuser::*};

use crate::debug;

/// It is used to control the lifecycle of the program.
pub struct App {}

impl App {
    /// Runs the application event loop.
    /// It will block this thread until all windows are closed.
    pub fn run() {
        let mut msg = MSG {
            hwnd: 0 as _,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        unsafe {
            debug!("Starting the event loop...");
            loop {
                if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    if msg.message == WM_QUIT {
                        debug!("Quitting the application...");
                        break;
                    }
                }
            }
        }
    }

    /// Notifies the application to quit.
    /// This will cause the event loop to break then the program will to exit.
    pub fn quit() {
        unsafe {
            PostQuitMessage(0);
        }
    }
}
