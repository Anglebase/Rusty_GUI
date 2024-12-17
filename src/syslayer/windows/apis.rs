use crate::{rect, Rect};
use std::{
    os::raw::c_void,
    ptr::{null, null_mut},
};
use winapi::{
    shared::windef::RECT,
    um::{wincon::FreeConsole, winuser::*},
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

pub fn get_absolute_rect(hwnd: *mut c_void) -> Rect {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetWindowRect(hwnd as _, &mut rect as _);
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

pub fn close_cmd() {
    unsafe {
        FreeConsole();
    }
}

pub fn set_no_auto_dpi_scale() {
    unsafe {
        SetProcessDPIAware();
    }
}

pub fn get_dpi_scale() -> f32 {
    unsafe { 96.0 / GetDpiForSystem() as f32 }
}

pub fn set_window_rect(hwnd: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    unsafe {
        SetWindowPos(
            hwnd as _,
            HWND_TOP,
            x,
            y,
            w,
            h,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn set_window_title(hwnd: *mut c_void, title: &str) {
    let title = title
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    unsafe {
        SetWindowTextW(hwnd as _, title.as_ptr());
    }
}

pub fn get_window_title(hwnd: *mut c_void) -> String {
    let len = unsafe { GetWindowTextLengthW(hwnd as _) };
    let mut title = vec![0u16; len as usize + 1];
    unsafe {
        GetWindowTextW(hwnd as _, title.as_mut_ptr(), len + 1);
    }
    String::from_utf16_lossy(&title)
}

pub fn update_window(hwnd: *mut c_void) {
    unsafe {
        RedrawWindow(
            hwnd as _,
            null(),
            null_mut(),
            RDW_INVALIDATE | RDW_UPDATENOW,
        );
    }
}

pub fn set_window_visible(hwnd: *mut c_void, visible: bool) {
    unsafe {
        ShowWindow(hwnd as _, if visible { SW_SHOW } else { SW_HIDE });
    }
}
