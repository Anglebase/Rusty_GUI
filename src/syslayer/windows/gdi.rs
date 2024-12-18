use core::panic;
use std::{f32::consts::PI, os::raw::c_void, ptr::null_mut};

use winapi::{
    shared::windef::RECT,
    um::{wingdi::*, winuser::*},
};

use crate::{rect, Color, PenStyle, Point, Rect};

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

pub fn draw_line(hdc: *mut c_void, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        MoveToEx(hdc as _, x1, y1, null_mut());
        LineTo(hdc as _, x2, y2);
    }
}

pub fn draw_rect(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    unsafe {
        MoveToEx(hdc as _, x, y, null_mut());
        LineTo(hdc as _, x + w, y);
        LineTo(hdc as _, x + w, y + h);
        LineTo(hdc as _, x, y + h);
        LineTo(hdc as _, x, y);
    }
}

pub fn draw_polyline(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a polyline");
    }
    unsafe { MoveToEx(hdc as _, points[0].x, points[0].y, null_mut()) };
    for Point { x, y } in points.iter() {
        unsafe { LineTo(hdc as _, *x, *y) };
    }
}

pub fn draw_polygon(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a polygon");
    }
    unsafe { MoveToEx(hdc as _, points[0].x, points[0].y, null_mut()) };
    for Point { x, y } in points.iter() {
        unsafe { LineTo(hdc as _, *x, *y) };
    }
    unsafe { LineTo(hdc as _, points[0].x, points[0].y) };
}

pub fn draw_arc(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (x1, y1, w, h) = rect.into();
    let (x2, y2) = (x1 + w, y1 + h);
    let (x, y) = (x1 + w / 2, y1 + h / 2);
    let (x3, y3) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (x4, y4) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        Arc(hdc as _, x1, y1, x2, y2, x3, y3, x4, y4);
    }
}

pub fn draw_pie(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        MoveToEx(hdc as _, x, y, null_mut());
        ArcTo(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
        LineTo(hdc as _, x, y);
    }
}

pub fn draw_ellipse(hdc: *mut c_void, rect: Rect) {
    draw_arc(hdc as _, rect, 0.0, 2.0 * PI);
}

pub fn draw_circle(hdc: *mut c_void, pos: Point, radius: i32) {
    let rect = rect!(pos.x - radius, pos.y - radius, 2 * radius, 2 * radius);
    draw_ellipse(hdc as _, rect);
}

pub fn draw_fill_rect(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        Rectangle(hdc as _, left, top, right, bottom);
    }
}

pub fn draw_fill_polygon(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a filled polygon");
    }
    unsafe {
        Polygon(
            hdc as _,
            points.as_ptr() as _,
            points.len().try_into().unwrap(),
        );
    }
}

pub fn draw_fille_pie(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        Pie(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
    }
}

pub fn draw_fille_ellipse(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        Ellipse(hdc as _, left, top, right, bottom);
    }
}

pub fn draw_fille_circle(hdc: *mut c_void, pos: Point, radius: i32) {
    let rect = rect!(pos.x - radius, pos.y - radius, 2 * radius, 2 * radius);
    draw_fille_ellipse(hdc as _, rect);
}

pub fn draw_xy_text(hdc: *mut c_void, pos: Point, text: &str) {
    let text = text
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let (x, y) = (pos.x, pos.y);
    unsafe {
        TextOutW(
            hdc as _,
            x,
            y,
            text.as_ptr() as _,
            (text.len() - 1).try_into().unwrap(),
        );
    }
}

pub fn draw_rect_text(hdc: *mut c_void, rect: Rect, text: &str) {
    let text = text
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let (x, y, w, h) = rect.into();
    let mut rect = RECT {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    };
    unsafe {
        DrawTextW(
            hdc as _,
            text.as_ptr() as _,
            (text.len() - 1).try_into().unwrap(),
            &mut rect,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    }
}
