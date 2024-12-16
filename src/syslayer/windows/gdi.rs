use std::os::raw::c_void;

use winapi::{
    shared::windef::RECT,
    um::{wingdi::*, winuser::FillRect},
};

use crate::{Color, PenStyle, Rect};

pub fn clear_device(hdc: *mut c_void, rect: Rect, color: Color) {
    unsafe {
        let rect = RECT {
            left: 0,
            top: 0,
            right: rect.size.width as i32,
            bottom: rect.size.height as i32,
        };
        let color = RGB(color.red, color.green, color.blue);
        let hbrush = CreateSolidBrush(color);
        FillRect(hdc as _, &rect, hbrush as _);
        DeleteObject(hbrush as _);
    }
}

pub enum PenParam {
    Normal(PenStyle, i32, Color),
}

pub fn delete_object(obj: *mut c_void) {
    unsafe {
        DeleteObject(obj as _);
    }
}

pub fn select_object(hdc: *mut c_void, obj: *mut c_void) -> *mut c_void {
    unsafe { SelectObject(hdc as _, obj as _) as *mut c_void }
}

pub fn new_pen_object(param: PenParam) -> *mut c_void {
    match param {
        PenParam::Normal(style, width, color) => {
            let color = RGB(color.red, color.green, color.blue);
            let style = match style {
                PenStyle::Solid => PS_SOLID,
                PenStyle::Dash => PS_DASH,
                PenStyle::Dot => PS_DOT,
                PenStyle::DashDot => PS_DASHDOT,
                PenStyle::DashDotDot => PS_DASHDOTDOT,
                PenStyle::Null => PS_NULL,
            };
            unsafe { CreatePen(style.try_into().unwrap(), width, color) as *mut c_void }
        }
    }
}

pub enum BrushParam {
    Solid(Color),
}

pub fn new_brush_object(param: BrushParam) -> *mut c_void {
    match param {
        BrushParam::Solid(color) => {
            let color = RGB(color.red, color.green, color.blue);
            unsafe { CreateSolidBrush(color) as *mut c_void }
        }
    }
}
