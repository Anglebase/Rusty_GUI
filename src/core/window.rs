use std::{os::raw::c_void, ptr::null_mut};

use crate::{
    create_window, disable_window, enable_window, get_absolute_rect, get_rect, get_window_title, register_hotkey_for_window, set_window_maximized, set_window_minimized, set_window_rect, set_window_restored, set_window_title, set_window_visible, show_and_update, update_window, HotKeyFlags, Rect
};

use super::{Ele, KeyCode, Widget};

#[derive(Clone)]
pub struct Window {
    pub(crate) hwnd: *mut c_void,
}

impl Default for Window {
    fn default() -> Self {
        Self { hwnd: null_mut() }
    }
}

impl Window {
    pub fn new<T: Ele>(title: &str, rect: Rect, parent: Option<&Window>, wp: &Widget<T>) -> Self {
        Self {
            hwnd: create_window(title, rect, parent, wp) as _,
        }
    }

    pub fn rect(&self) -> Rect {
        get_rect(self.hwnd)
    }

    pub fn title(&self) -> String {
        get_window_title(self.hwnd)
    }

    pub fn absrect(&self) -> Rect {
        get_absolute_rect(self.hwnd)
    }

    pub fn update(&self) {
        update_window(self.hwnd);
    }

    pub fn set_rect(&self, rect: Rect) {
        set_window_rect(self.hwnd, rect);
    }

    pub fn set_title(&self, title: &str) {
        set_window_title(self.hwnd, title);
    }

    pub fn set_visible(&self, visible: bool) {
        set_window_visible(self.hwnd, visible);
    }

    pub fn show(&self) {
        show_and_update(self.hwnd);
    }

    pub fn hide(&self) {
        self.set_visible(false);
    }

    pub fn minimize(&self) {
        set_window_minimized(self.hwnd);
    }

    pub fn maximize(&self) {
        set_window_maximized(self.hwnd);
    }

    pub fn restore(&self) {
        set_window_restored(self.hwnd);
    }

    pub fn disable(&self) {
        disable_window(self.hwnd);
    }

    pub fn enable(&self) {
        enable_window(self.hwnd);
    }

    pub fn register_hotkey(&self, id: i32, modifiers: HotKeyFlags, key: KeyCode) {
        if id < 0 || id > 0xBFFF {
            panic!("Invalid hotkey id: {}", id);
        }
        register_hotkey_for_window(self.hwnd, id, key, modifiers);
    }
}
