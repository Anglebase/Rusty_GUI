use std::{any::type_name, ptr::null_mut};

use winapi::{
    shared::{
        ntdef::WCHAR,
        windef::{HBRUSH, HWND, POINT},
    },
    um::{libloaderapi::GetModuleHandleW, winuser::*},
};

use crate::{Ele, Rect, Widget, Window};

use super::winproc;

pub fn register_class(class_name: &Vec<WCHAR>) {
    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
        let wndcls = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
            lpfnWndProc: Some(winproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr() as _,
            hIconSm: null_mut(),
        };
        let mut wnd = wndcls.clone();
        if GetClassInfoExW(hinstance, class_name.as_ptr(), &mut wnd as *mut _) == 0 {
            RegisterClassExW(&wndcls);
        }
    }
}

pub fn create_window<T: Ele>(
    title: &str,
    rect: Rect,
    parent: Option<&Window>,
    wp: &Widget<T>,
) -> HWND {
    let class_name = type_name::<T>()
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<WCHAR>>();
    register_class(&class_name);
    let title = title
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<WCHAR>>();
    unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            if let None = parent {
                WS_OVERLAPPEDWINDOW
            } else {
                WS_CHILD
            },
            rect.pos.x,
            rect.pos.y,
            rect.size.width,
            rect.size.height,
            if let Some(window) = parent {
                window.hwnd as HWND
            } else {
                null_mut()
            },
            null_mut(),
            null_mut(),
            wp.addr() as _,
        )
    }
}

pub fn event_loop() {
    let mut msg = MSG {
        hwnd: null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };
    unsafe {
        loop {
            if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                if msg.message == WM_QUIT {
                    break;
                }
            }
        }
    }
}
