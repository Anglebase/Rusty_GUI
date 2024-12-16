use crate::{rect, Rect};
use std::os::raw::c_void;
use winapi::{
    shared::windef::RECT,
    um::winuser::*,
};

pub fn get_rect(hwnd: *mut c_void) -> Rect {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetClientRect(hwnd as _, &mut rect as _);
    }
    rect!(
        rect.left,
        rect.top,
        rect.right - rect.left,
        rect.bottom - rect.top
    )
}

pub fn show_and_update(hwnd: *mut c_void) {
    unsafe {
        ShowWindow(hwnd as _, SW_SHOW);
        UpdateWindow(hwnd as _);
    }
}

pub fn notifier_exit(code: i32) {
    unsafe {
        PostQuitMessage(code);
    }
}
