//! This file is contains the implementation of the Window struct and its methods.

use std::{os::raw::c_void, ptr::null_mut};

use winapi::um::winuser::EnumChildWindows;

use crate::*;

use super::{Ele, KeyCode, Widget};

/// The Window struct represents a window on the screen.
/// It is the base of all GUI element.
/// Any GUI element must have a field of type Window. It is used to implement trait `AsWindow`.
#[derive(Clone)]
pub struct Window {
    pub(crate) hwnd: *mut c_void,
}

impl Default for Window {
    /// This method returns a empty Window struct as a placeholder.
    fn default() -> Self {
        Self { hwnd: null_mut() }
    }
}

impl Window {
    /// Create a new Window with the given title, rect, parent window, and widget.
    /// The parent window can be None if the window is a top-level window.
    pub fn new<T: Ele>(title: &str, rect: Rect, parent: Option<&Window>, wp: &Widget<T>) -> Self {
        Self {
            hwnd: create_window(title, rect, parent, wp) as _,
        }
    }

    /// Get the area of the window 
    pub fn rect(&self) -> Rect {
        get_rect(self.hwnd)
    }

    /// Get the title of the window 
    pub fn title(&self) -> String {
        get_window_title(self.hwnd)
    }

    /// Get the absolute rect of the window relative to the screen, including the title bar and borders.
    pub fn absrect(&self) -> Rect {
        get_absolute_rect(self.hwnd)
    }

    /// Update the window.
    /// If window needs to be redrawn, this method should be called.
    pub fn update(&self) {
        update_window(self.hwnd);
    }

    /// Set the rect of the window.
    pub fn set_rect(&self, rect: Rect) {
        set_window_rect(self.hwnd, rect);
    }

    /// Set the title of the window.
    pub fn set_title(&self, title: &str) {
        set_window_title(self.hwnd, title);
    }

    /// Set the visibility of the window.
    pub fn set_visible(&self, visible: bool) {
        set_window_visible(self.hwnd, visible);
    }

    /// Show the window and update it.
    pub fn show(&self) {
        show_and_update(self.hwnd);
    }

    /// Hide the window and update it.
    pub fn hide(&self) {
        self.set_visible(false);
    }

    /// Minimize the window.
    pub fn minimize(&self) {
        set_window_minimized(self.hwnd);
    }

    /// Maximize the window.
    pub fn maximize(&self) {
        set_window_maximized(self.hwnd);
    }

    /// Restore the window from maximized or minimized state.
    pub fn restore(&self) {
        set_window_restored(self.hwnd);
    }

    /// Disable the window.
    pub fn disable(&self) {
        disable_window(self.hwnd);
    }

    /// Enable the window.
    pub fn enable(&self) {
        enable_window(self.hwnd);
    }

    /// Register a hotkey for the window.
    /// # Panics
    /// If the hotkey `id` is not between 0 and 0xBFFF (Out of range is invalid).
    pub fn register_hotkey(&self, id: i32, modifiers: HotKeyFlags, key: KeyCode) {
        if id < 0 || id > 0xBFFF {
            panic!("Invalid hotkey id: {}", id);
        }
        register_hotkey_for_window(self.hwnd, id, key, modifiers);
    }

    /// Create a timer for the window.
    pub fn set_timer(&self, id: usize, interval: u32) {
        set_window_timer(self.hwnd, id, interval);
    }

    /// Kill a timer for the window.
    pub fn kill_timer(&self, id: usize) {
        kill_window_timer(self.hwnd, id);
    }

    /// Set the window style.
    /// # *INSTABILITY* !!!
    /// # *UNTESTED* !!!
    pub fn set_style(&self, style: WindowStyle) {
        set_window_style(self.hwnd, style);
    }

    /// Get the window style.
    /// # *INSTABILITY* !!!
    /// # *UNTESTED* !!!
    pub fn get_style(&self) -> WindowStyle {
        get_window_style(self.hwnd)
    }

    /// For each child window of the window, call the given function.
    /// It usually used with trait `Userdata`.
    pub fn foreach(&self, mut f: Box<dyn FnMut(&mut dyn Ele)>) {
        unsafe {
            EnumChildWindows(
                self.hwnd as _,
                Some(enum_windows_callback),
                &mut f as *mut _ as _,
            );
        }
    }
}
