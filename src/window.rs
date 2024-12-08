use std::ptr::null_mut;

use winapi::{shared::windef::HWND, um::winuser::*};

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

    pub fn set_parent(&self, parent: &Window) {
        unsafe {
            SetParent(self.hwnd, parent.hwnd);
        }
    }
}
