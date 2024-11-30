use std::ptr::{null, null_mut};

/// This file contains the management interface functions for the program.
/// author: Anglebase (https://github.com/Anglebase)
/// ----------------------------------------------------------------------
use winapi::{shared::windef::*, um::winuser::*};

use crate::core::G_MAP;

#[allow(unused)]
pub struct App {}

impl App {
    /// Runs the main loop of the program.
    /// This function is blocking and will not return until the program is closed.
    /// It should be called at the end of the `main` function.
    /// It should be noted that before this function is called,
    /// you must ensure that a window has been created.
    ///
    /// # Example
    /// ```
    /// use rusty_gui::App;
    /// 
    /// fn main() {
    ///     //...Other initialization code...
    ///     App::run();
    /// }
    /// ```
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
            loop {
                for key in G_MAP.lock().unwrap().as_ref().unwrap().keys() {
                    RedrawWindow(
                        *key as HWND,
                        null(),
                        null_mut(),
                        RDW_INTERNALPAINT | RDW_INVALIDATE,
                    );
                }
                let ret = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
                if ret != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                if msg.message == WM_QUIT {
                    break;
                }
            }
        }
    }

    /// When this function is called, it will notify the event loop to exit.
    /// It will not immediately terminate the program.
    ///
    /// # Example
    /// ```
    /// use rusty_gui::*;
    /// 
    /// struct MyWindow {}
    /// impl WinProc for MyWindow {
    ///     fn button_up(&mut self, btn: Button) {
    ///         if let Button::Left(_) = btn  {
    ///             App::exit(); // this will cause the program to exit.
    ///         }
    ///     }
    /// }
    /// 
    /// fn main() {
    ///     let window = Window::new(Box::new(MyWindow {}), None);
    ///     window.show();
    ///     App::run();
    /// }
    ///
    /// ```
    #[allow(unused)]
    pub fn exit() {
        unsafe { PostQuitMessage(0) };
    }
}
