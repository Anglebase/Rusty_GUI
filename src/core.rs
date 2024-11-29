use crate::winproc::WinProc;
/// This file contains the core implementation of this crate.
/// author: Anglebase (https://github.com/Anglebase)
/// ---------------------------------------------------------
use std::{collections::HashMap, sync::Mutex};
use winapi::{
    shared::{minwindef::*, windef::*},
    um::{libloaderapi::*, winnt::*, winuser::*},
};

pub(crate) fn string_to_wchar(s: &str) -> Vec<WCHAR> {
    let mut result = Vec::new();
    for c in s.chars() {
        result.push(c as WCHAR);
    }
    result.push(0);
    result
}

// Trait
pub(crate) unsafe fn register_window_class(name: &str, winproc: WNDPROC) -> ATOM {
    let class_name = string_to_wchar(name);
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: winproc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: GetModuleHandleW(std::ptr::null()),
        hIcon: std::ptr::null_mut(),
        hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
        hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
        lpszMenuName: std::ptr::null_mut(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: std::ptr::null_mut(),
    };
    RegisterClassExW(&class)
}

// Window new
pub(crate) unsafe fn create_window(
    class_name: &str,
    title: &str,
    width: i32,
    height: i32,
    parent: HWND,
) -> HWND {
    let window_name = string_to_wchar(title);
    let hwnd = CreateWindowExW(
        0,
        string_to_wchar(class_name).as_ptr(),
        window_name.as_ptr(),
        if !parent.is_null() {
            WS_CHILD
        } else {
            WS_OVERLAPPEDWINDOW
        },
        100,
        100,
        width,
        height,
        parent,
        std::ptr::null_mut(),
        GetModuleHandleW(std::ptr::null()),
        std::ptr::null_mut(),
    );
    hwnd
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

#[allow(unused)]
pub struct Graphics {
    pub(crate) hdc: HDC,
    pub(crate) hwnd: HWND,
}

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct Window {
    pub(crate) hwnd: HWND,
}

pub(crate) const CLASS_NAME: &str = "rusty_gui_window_class";
pub(crate) static mut G_MAP: Mutex<Option<HashMap<HWND, Box<dyn WinProc>>>> = Mutex::new(None);

pub(crate) fn gmap_init() {
    unsafe {
        *G_MAP.lock().unwrap() = Some(HashMap::new());
    }
}
pub(crate) fn gmap_insert(hwnd: HWND, proc: Box<dyn WinProc>) {
    unsafe {
        let mut map = G_MAP.lock().unwrap();
        map.as_mut().unwrap().insert(hwnd, proc);
    }
}
pub(crate) fn gmap_remove(hwnd: HWND) {
    unsafe {
        let mut map = G_MAP.lock().unwrap();
        map.as_mut().unwrap().remove(&hwnd);
    }
}
pub(crate) fn gmap_is_null() -> bool {
    unsafe { G_MAP.lock().unwrap().is_none() }
}

pub(crate) static mut G_MAINWINDOW: Mutex<Option<Window>> = Mutex::new(None);
pub(crate) unsafe extern "system" fn gwndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT {
                hdc: std::ptr::null_mut(),
                fErase: 0,
                rcPaint: std::mem::zeroed(),
                fRestore: 0,
                fIncUpdate: 0,
                rgbReserved: [0; 32],
            };
            let hdc = BeginPaint(hwnd, &mut ps);
            let mut graphics = Graphics { hdc, hwnd };
            {
                G_MAP.lock().unwrap().as_ref().unwrap()[&hwnd].draw(&mut graphics);
            }
            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            gmap_remove(hwnd);
            if G_MAINWINDOW.lock().unwrap().as_ref().unwrap().hwnd == hwnd {
                PostQuitMessage(0);
            }
            0
        }
        WM_LBUTTONDOWN => {
            {
                G_MAP.lock().unwrap().as_ref().unwrap()[&hwnd]
                    .left_button_down(LOWORD(lparam as u32).into(), HIWORD(lparam as u32).into());
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
