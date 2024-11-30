use crate::core::string_to_wchar;
use std::ptr::null_mut;

/// This file contains system API interactions and encapsulation based on device handles.
/// author: Anglebase (https://github.com/Anglebase)
/// -------------------------------------------------------------------------------------
use crate::*;
use winapi::{
    shared::windef::*,
    um::{wingdi::*, winuser::*},
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr $(,)?) => {
        Color {
            red: $r,
            green: $g,
            blue: $b,
        }
    };
}

impl Color {
    pub const BLACK: Color = rgb! { 0, 0, 0 };
    pub const WHITE: Color = rgb! { 255, 255, 255 };
    pub const RED: Color = rgb! { 255, 0, 0 };
    pub const GREEN: Color = rgb! { 0, 255, 0 };
    pub const BLUE: Color = rgb! { 0, 0, 255 };
    pub const YELLOW: Color = rgb! { 255, 255, 0 };
    pub const CYAN: Color = rgb! { 0, 255, 255 };
    pub const MAGENTA: Color = rgb! { 255, 0, 255 };
}

pub struct Pen {
    pub(crate) hpen: HPEN,
}
impl Drop for Pen {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hpen as HGDIOBJ);
        }
    }
}

pub enum PenStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
}

impl Pen {
    pub fn new(ps: PenStyle, width: i32, color: Color) -> Self {
        let ps = match ps {
            PenStyle::Solid => PS_SOLID,
            PenStyle::Dash => PS_DASH,
            PenStyle::Dot => PS_DOT,
            PenStyle::DashDot => PS_DASHDOT,
            PenStyle::DashDotDot => PS_DASHDOTDOT,
            PenStyle::Null => PS_NULL,
        };
        Self {
            hpen: unsafe { CreatePen(ps as i32, width, RGB(color.red, color.green, color.blue)) },
        }
    }
}

pub struct Brush {
    pub(crate) hbrush: HBRUSH,
}
impl Drop for Brush {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hbrush as HGDIOBJ);
        }
    }
}
impl Brush {
    pub fn new(color: Color) -> Self {
        Self {
            hbrush: unsafe { CreateSolidBrush(RGB(color.red, color.green, color.blue)) },
        }
    }
}

pub struct Font {
    pub(crate) hfont: HFONT,
}
pub enum FontWeight {
    Dontcare = FW_DONTCARE as isize,
    Thin = FW_THIN as isize,
    ExtraLight = FW_EXTRALIGHT as isize,
    Light = FW_LIGHT as isize,
    Normal = FW_NORMAL as isize,
    Medium = FW_MEDIUM as isize,
    SemiBold = FW_SEMIBOLD as isize,
    Bold = FW_BOLD as isize,
    ExtraBold = FW_EXTRABOLD as isize,
    Black = FW_BLACK as isize,
}
pub struct FontStyle {
    pub size: i32,
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub font: String,
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hfont as HGDIOBJ);
        }
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            size: 16,
            weight: FontWeight::Normal,
            italic: false,
            underline: false,
            strikeout: false,
            font: String::from("宋体"),
        }
    }
}

impl Font {
    pub fn new(style: FontStyle) -> Self {
        let hfont = unsafe {
            CreateFontW(
                style.size,
                0,
                0,
                0,
                style.weight as i32,
                style.italic as u32,
                style.underline as u32,
                style.strikeout as u32,
                DEFAULT_CHARSET,
                OUT_DEFAULT_PRECIS,
                CLIP_DEFAULT_PRECIS,
                DEFAULT_QUALITY,
                DEFAULT_PITCH | FF_DONTCARE,
                string_to_wchar(style.font.as_str()).as_ptr(),
            )
        };
        Self { hfont }
    }
}

impl Graphics {
    pub fn apply_pen(&self, pen: &Pen) {
        unsafe {
            SelectObject(self.hdc, pen.hpen as HGDIOBJ);
        }
    }
    pub fn apply_brush(&self, brush: &Brush) {
        unsafe {
            SelectObject(self.hdc, brush.hbrush as HGDIOBJ);
        }
    }
    pub fn apply_font(&self, font: &Font) {
        unsafe {
            SelectObject(self.hdc, font.hfont as HGDIOBJ);
        }
    }

    pub fn full_clear(&mut self, window: &Window, color: Color) {
        unsafe {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetClientRect(window.hwnd, &mut rect);
            let bs = CreateSolidBrush(RGB(color.red, color.green, color.blue));
            FillRect(self.hdc, &rect, bs);
            DeleteObject(bs as HGDIOBJ);
        }
    }
    pub fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc, p1.x, p1.y, null_mut());
            LineTo(self.hdc, p2.x, p2.y);
        }
    }
    pub fn text(&self, text: &str, p: Point) {
        let s = string_to_wchar(text);
        unsafe {
            TextOutW(
                self.hdc,
                p.x,
                p.y,
                s.as_ptr(),
                s.len() as i32 - 1,
            );
        }
    }
}
