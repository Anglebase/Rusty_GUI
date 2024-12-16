use std::{os::raw::c_void, ptr::null_mut};

use crate::{create_window, get_rect, show_and_update, Rect};

use super::{Ele, Widget};

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

    pub fn show(&self) {
        show_and_update(self.hwnd);
    }
}
