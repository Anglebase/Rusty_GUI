/// This file contains the system API interaction interface based on window handle (HWND).
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------------------------------------

use winapi::um::winuser::*;
use crate::core::*;
use crate::WinProc;

impl Window {
    #[allow(unused)]
    pub fn new(wimpl: Box<impl WinProc + 'static>, parent: Option<&Window>) -> Self {
        unsafe {
            if gmap_is_null() {
                gmap_init();
                register_window_class(CLASS_NAME, Some(gwndproc));
            }
            let parent_hwnd = if let Some(parent) = parent {
                parent.hwnd
            } else {
                if G_MAINWINDOW.lock().unwrap().as_ref().is_some() {
                    panic!("Only one main window is allowed");
                }
                std::ptr::null_mut()
            };
            let hwnd = create_window(CLASS_NAME, "Rusty GUI Window", 800, 600, parent_hwnd);
            gmap_insert(hwnd, wimpl);
            if G_MAINWINDOW.lock().unwrap().as_ref().is_none() {
                *G_MAINWINDOW.lock().unwrap() = Some(Window { hwnd });
            }
            Self { hwnd }
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            UpdateWindow(self.hwnd);
        }
    }

    pub fn resize(&self, size: Size) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                0,
                0,
                size.width,
                size.height,
                SWP_NOMOVE | SWP_NOZORDER,
            );
        }
    }
    pub fn move_to(&self, pos: Point) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                pos.x,
                pos.y,
                0,
                0,
                SWP_NOSIZE | SWP_NOZORDER,
            );
        }
    }
}
