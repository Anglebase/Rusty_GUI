use std::ptr::null_mut;

use winapi::{
    shared::{ntdef::LPCWSTR, windef::*},
    um::wingdi::*,
};

use crate::{Color, Point};

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
        let font = style
            .font
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr() as LPCWSTR;
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
                font,
            )
        };
        Self { hfont }
    }
}

#[allow(unused)]
pub struct Graph {
    pub(crate) hdc: HDC,
}

impl Graph {
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
    pub fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc, p1.x, p1.y, null_mut());
            LineTo(self.hdc, p2.x, p2.y);
        }
    }
    pub fn text(&self, text: &str, p: Point) {
        let len = text.len();
        let text = text
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr() as LPCWSTR;
        unsafe {
            TextOutW(self.hdc, p.x, p.y, text, len as i32 - 1);
        }
    }
}
