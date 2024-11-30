use crate::core::*;
use crate::p;
use crate::s;
use crate::App;
use crate::WinProc;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
/// This file contains the system API interaction interface based on window handle (HWND).
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------------------------------------
use winapi::um::winuser::*;

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
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .unwrap()
                .init(&mut Window { hwnd });
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

    pub fn set_title(&self, title: &str) {
        let h = self.hwnd as usize;
        let title = title.to_string();
        let task = move || unsafe {
            SetWindowTextW(h as HWND, string_to_wchar(title.as_str()).as_ptr());
        };
        App::push(task);
    }

    pub fn set_enabled(&self, enabled: bool) {
        let h = self.hwnd as usize;
        let task = move || unsafe {
            EnableWindow(h as HWND, if enabled { TRUE } else { FALSE });
        };
        App::push(task);
    }

    pub fn get_client_rect(&self) -> Rect {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            GetClientRect(self.hwnd, &mut rect);
        }
        Rect {
            pos: p!(rect.left, rect.top),
            size: s!(rect.right - rect.left, rect.bottom - rect.top),
        }
    }
}
