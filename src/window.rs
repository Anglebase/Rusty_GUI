use std::ptr::null_mut;

use winapi::{
    shared::{ntdef::LPCWSTR, windef::HWND},
    um::winuser::*,
};

pub struct Window {
    pub(crate) hwnd: HWND,
}

impl Window {
    /// Show the window and update it.
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            UpdateWindow(self.hwnd);
        }
    }
    /// Hide the window.
    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    /// Redraw the window.
    /// The child windows are also redrawn.
    pub fn update(&self) {
        unsafe {
            RedrawWindow(
                self.hwnd,
                null_mut(),
                null_mut(),
                RDW_INVALIDATE | RDW_UPDATENOW,
            );
        }
    }
    /// Set the parent window of the window.
    pub fn set_parent(&self, parent: &Window) {
        unsafe {
            SetParent(self.hwnd, parent.hwnd);
        }
    }
    /// Set the title of the window.
    pub fn set_title(&self, title: &str) {
        let title = title
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr() as LPCWSTR;
        unsafe {
            SetWindowTextW(self.hwnd, title);
        }
    }

    /// Create a timer for the window.
    /// It will trigger the `WinProc::timer` function every `time` milliseconds.
    /// The `timer_id` is used to identify the timer.
    pub fn set_timer(&self, time: u32, timer_id: usize) -> bool {
        unsafe {
            let ret = SetTimer(self.hwnd, timer_id.into(), time, None);
            if ret == 0 {
                return false;
            }
            true
        }
    }

    /// Kill the timer with the given `timer_id`.
    pub fn kill_timer(&self, timer_id: usize) -> bool {
        unsafe {
            let ret = KillTimer(self.hwnd, timer_id.into());
            if ret == 0 {
                return false;
            }
            true
        }
    }
}
