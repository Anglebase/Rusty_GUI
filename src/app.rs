/// This file contains the management interface functions for the program.
/// author: Anglebase (https://github.com/Anglebase)
/// ----------------------------------------------------------------------

use winapi::{shared::windef::*, um::winuser::*};

#[allow(unused)]
pub struct App {}

impl App {
    /// Runs the main loop of the program.
    /// This function is blocking and will not return until the program is closed.
    /// It should be called at the end of the `main` function.
    /// 
    /// # Example
    /// ```
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
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    /// When this function is called, it will notify the event loop to exit.
    /// It will not immediately terminate the program.
    /// 
    /// # Example
    /// ```
    /// struct MyWindow {}
    /// impl WinProc for MyWindow {
    ///     fn left_button_up(&self, window: Window) {
    ///         App::exit(); // this will cause the program to exit.
    ///     }
    /// }
    /// 
    /// ```
    #[allow(unused)]
    pub fn exit() {
        unsafe { PostQuitMessage(0) };
    }
}
