use std::ptr::null_mut;

use winapi::{
    shared::{ntdef::LPCWSTR, windef::HWND},
    um::winuser::*,
};

/// Window object
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

    /// Notify the window that it needs to be redrawn.
    /// If `include_children` is true, the entire window and its children will be redrawn.
    /// Otherwise, only the client area of the window will be redrawn excluding its children.
    pub fn update(&self, include_children: bool) {
        unsafe {
            RedrawWindow(
                self.hwnd,
                null_mut(),
                null_mut(),
                RDW_INVALIDATE
                    | RDW_ERASE
                    | RDW_UPDATENOW
                    | if include_children {
                        RDW_ALLCHILDREN
                    } else {
                        RDW_NOCHILDREN
                    },
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

    /// Get the title of the window.
    pub fn title(&self) -> String {
        let len = unsafe { GetWindowTextLengthW(self.hwnd) };
        let mut buf = vec![0u16; len as usize + 1];
        unsafe {
            GetWindowTextW(self.hwnd, buf.as_mut_ptr(), len + 1);
        }
        String::from_utf16_lossy(&buf)
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

    /// Get the current focus window.
    pub fn get_fouse() -> Option<Window> {
        let hwnd = unsafe { GetFocus() };
        if hwnd == null_mut() {
            None
        } else {
            Some(Window { hwnd })
        }
    }

    /// Check if the window has the fouse.
    pub fn is_fouse(&self) -> bool {
        let hwnd = unsafe { GetFocus() };
        hwnd == self.hwnd
    }
}
