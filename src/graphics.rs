use std::ptr::null_mut;

use winapi::{
    shared::{ntdef::LPCWSTR, windef::*},
    um::wingdi::*,
};

use crate::{p, Color, Point, Rect};

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
                PROOF_QUALITY,
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
    /// Apply a pen for the graphics context.
    pub fn apply_pen(&self, pen: &Pen) {
        unsafe {
            SelectObject(self.hdc, pen.hpen as HGDIOBJ);
        }
    }
    /// Apply a brush for the graphics context.
    pub fn apply_brush(&self, brush: &Brush) {
        unsafe {
            SelectObject(self.hdc, brush.hbrush as HGDIOBJ);
        }
    }
    /// Apply a font for the graphics context.
    pub fn apply_font(&self, font: &Font) {
        unsafe {
            SelectObject(self.hdc, font.hfont as HGDIOBJ);
        }
    }
    /// Set the text color for the graphics context.
    pub fn set_text_color(&self, color: Color) {
        unsafe {
            SetTextColor(self.hdc, RGB(color.red, color.green, color.blue));
        }
    }
    /// Set the background color for the graphics context.
    /// It is used as the background color of the text.
    pub fn set_bk_color(&self, color: Color) {
        unsafe {
            SetBkColor(self.hdc, RGB(color.red, color.green, color.blue));
        }
    }
    /// Set the background mode for the graphics context.
    /// If `mode` is true, the background is transparent.
    pub fn set_bk_transparent(&self, mode: bool) {
        unsafe {
            SetBkMode(
                self.hdc,
                if mode {
                    TRANSPARENT as i32
                } else {
                    OPAQUE as i32
                },
            );
        }
    }

    /// Draw a line from `p1` to `p2`.
    /// Its style is determined by the current pen.
    pub fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc, p1.x, p1.y, null_mut());
            LineTo(self.hdc, p2.x, p2.y);
        }
    }
    /// Draw an arc with the given arguments.
    /// Its style is determined by the current pen.
    /// Parameter `pos` is the center of the arc,
    /// `xr` and `yr` are the x direction and y direction radii of the arc,
    /// `start` and `end` are the start and end angles of the arc in radians.
    /// The angles are measured from the x-axis(horizontal direction) in a counter-clockwise direction.
    pub fn arc(&self, pos: Point, xr: i32, yr: i32, start: f64, end: f64) {
        unsafe {
            Arc(
                self.hdc,
                pos.x - xr,
                pos.y - yr,
                pos.x + xr,
                pos.y + yr,
                pos.x + (xr as f64 * start.cos()) as i32,
                pos.y - (yr as f64 * start.sin()) as i32,
                pos.x + (xr as f64 * end.cos()) as i32,
                pos.y - (yr as f64 * end.sin()) as i32,
            );
        }
    }
    pub fn pie(&self, pos: Point, xr: i32, yr: i32, start: f64, end: f64) {
        self.arc(pos, xr, yr, start, end);
        let p1 = p!(
            pos.x + (xr as f64 * start.cos()) as i32,
            pos.y - (yr as f64 * start.sin()) as i32
        );
        let p2 = p!(
            pos.x + (xr as f64 * end.cos()) as i32,
            pos.y - (yr as f64 * end.sin()) as i32
        );
        self.line(pos, p1);
        self.line(pos, p2);
    }
    pub fn ellipse(&self, pos: Point, xr: i32, yr: i32) {
        self.arc(pos, xr, yr, 0.0, 0.0);
    }
    pub fn circle(&self, pos: Point, r: i32) {
        self.ellipse(pos, r, r);
    }
    /// Draw a rectangle wireframe.
    /// Its style is determined by the current pen.
    pub fn rect(&self, rect: Rect) {
        let lt = rect.pos;
        let rt = Point {
            x: lt.x + rect.size.width,
            y: lt.y,
        };
        let rb = Point {
            x: lt.x + rect.size.width,
            y: lt.y + rect.size.height,
        };
        let lb = Point {
            x: lt.x,
            y: lt.y + rect.size.height,
        };
        self.line(lt, rt);
        self.line(rt, rb);
        self.line(rb, lb);
        self.line(lb, lt);
    }
    /// Draw a filled rectangle.
    /// Its outline style is determined by the current pen,
    /// and its fill style is determined by the current brush.
    pub fn fillrect(&self, rect: Rect) {
        unsafe {
            Rectangle(
                self.hdc,
                rect.pos.x,
                rect.pos.y,
                rect.pos.x + rect.size.width,
                rect.pos.y + rect.size.height,
            );
        }
    }
    pub fn fill_ellipse(&self, pos: Point, xr: i32, yr: i32) {
        unsafe {
            Ellipse(self.hdc, pos.x - xr, pos.y - yr, pos.x + xr, pos.y + yr);
        }
    }
    /// Draw text at the given position.
    /// Its font and color are determined by the current font and text color.
    pub fn text(&self, text: &str, p: Point) {
        let text = text
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        unsafe {
            TextOutW(self.hdc, p.x, p.y, text.as_ptr() as _, (text.len() - 1) as _);
        }
    }
}
