use std::ptr::null_mut;

use winapi::{
    shared::{ntdef::LPCWSTR, windef::*},
    um::{wingdi::*, winuser::*},
};

use crate::{Color, Point, Rect};

/// The pen of the graphics context.
pub struct Pen {
    pub(crate) hpen: HPEN,
}
impl Drop for Pen {
    /// Drop the pen and release its resources.
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hpen as HGDIOBJ);
        }
    }
}
/// The style of the pen.
pub enum PenStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
}

impl Pen {
    /// Create a new pen with the given style, width, and color.
    /// `ps` is the style of the pen,
    /// `width` is the width of the pen,
    /// `color` is the color of the pen.
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

/// The brush of the graphics context.
pub struct Brush {
    pub(crate) hbrush: HBRUSH,
}
impl Drop for Brush {
    /// Drop the brush and release its resources.
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hbrush as HGDIOBJ);
        }
    }
}
impl Brush {
    /// Create a new solid brush with the given color.
    /// `color` is the color of the brush.
    pub fn new(color: Color) -> Self {
        Self {
            hbrush: unsafe { CreateSolidBrush(RGB(color.red, color.green, color.blue)) },
        }
    }
}

/// The font of the graphics context.
pub struct Font {
    pub(crate) hfont: HFONT,
}
/// The enum of font weights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontWeight {
    Dontcare = FW_DONTCARE as isize,     // Use the default weight.
    Thin = FW_THIN as isize,             // Thin font weight.
    ExtraLight = FW_EXTRALIGHT as isize, // Extra light font weight.
    Light = FW_LIGHT as isize,           // Light font weight.
    Normal = FW_NORMAL as isize,         // Normal font weight.
    Medium = FW_MEDIUM as isize,         // Medium font weight.
    SemiBold = FW_SEMIBOLD as isize,     // Semi-bold font weight.
    Bold = FW_BOLD as isize,             // Bold font weight.
    ExtraBold = FW_EXTRABOLD as isize,   // Extra bold font weight.
    Black = FW_BLACK as isize,           // Black font weight.
}
/// The struct of font style.
#[derive(Clone)]
pub struct FontStyle {
    pub size: i32,          // The size of the font.
    pub weight: FontWeight, // The weight of the font.
    pub italic: bool,       // The italic status of the font.
    pub underline: bool,    // The underline status of the font.
    pub strikeout: bool,    // The strikeout status of the font.
    pub fontfamily: String, // The font family of the font.
}

impl FontStyle {
    /// Set the font family for the font style.
    pub fn set_fontfamily(&mut self, family: &str) -> Self {
        self.fontfamily = family.to_string();
        self.clone()
    }
    /// Set the font size for the font style.
    pub fn set_size(&mut self, size: i32) -> Self {
        self.size = size;
        self.clone()
    }
    /// Set the font weight type for the font style.
    pub fn set_weight(&mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self.clone()
    }
    /// Set the font italic status for the font style.
    pub fn set_italic(&mut self, italic: bool) -> Self {
        self.italic = italic;
        self.clone()
    }
    /// Set the font underline status for the font style.
    pub fn set_underline(&mut self, underline: bool) -> Self {
        self.underline = underline;
        self.clone()
    }
    /// Set the font strikeout status for the font style.
    pub fn set_strikeout(&mut self, strikeout: bool) -> Self {
        self.strikeout = strikeout;
        self.clone()
    }
}

impl Drop for Font {
    /// Drop the font and release its resources.
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hfont as HGDIOBJ);
        }
    }
}

impl Default for FontStyle {
    /// Create a default font style.
    fn default() -> Self {
        Self {
            size: 16,
            weight: FontWeight::Normal,
            italic: false,
            underline: false,
            strikeout: false,
            fontfamily: String::from("宋体"),
        }
    }
}

impl Font {
    /// Create a new font with the given font style.
    pub fn new(style: &FontStyle) -> Self {
        let font = style
            .fontfamily
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

pub struct TextAlign;
type TextAligns = u32;
impl TextAlign {
    pub const BASELINE: u32 = TA_BASELINE; // Align the point to the baseline of the text.
    pub const LEFT: u32 = TA_LEFT; // Align the point to the left of the text.
    pub const CENTER: u32 = TA_CENTER; // Align the point to the center of the text.
    pub const RIGHT: u32 = TA_RIGHT; // Align the point to the right of the text.
    pub const TOP: u32 = TA_TOP; // Align the point to the top of the text.
    pub const BOTTOM: u32 = TA_BOTTOM; // Align the point to the bottom of the text.
}
pub struct TextFomat;
type TextFormats = u32;
impl TextFomat {
    pub const BOTTOM: u32 = DT_BOTTOM; // Bottom alignment.
    pub const VCENTER: u32 = DT_VCENTER; // Vertical Center alignment.
    pub const TOP: u32 = DT_TOP; // Top alignment.
    pub const LEFT: u32 = DT_LEFT; // Left alignment.
    pub const CENTER: u32 = DT_CENTER; // Horizontal Center alignment.
    pub const RIGHT: u32 = DT_RIGHT; // Right alignment.

    pub const END_ELLIPSIS: u32 = DT_END_ELLIPSIS; // End ellipsis.
    pub const MIDDLE_ELLIPSIS: u32 = DT_PATH_ELLIPSIS; // Middle ellipsis.

    pub const SINGLE_LINE: u32 = DT_SINGLELINE; // Single line.\

    pub const ATCENTER: u32 = DT_CENTER | DT_VCENTER | DT_SINGLELINE; // Center alignment.
}

/// The graphics context.
pub struct Graph {
    pub(crate) hdc: HDC,
}

impl Graph {
    /// Apply a pen for the graphics context.
    /// Return a Pen object that can be used to restore the previous pen.
    pub fn apply_pen(&self, pen: &Pen) -> Pen {
        let hpen = unsafe { SelectObject(self.hdc, pen.hpen as HGDIOBJ) } as HPEN;
        Pen { hpen }
    }
    /// Apply a brush for the graphics context.
    /// Return a Brush object that can be used to restore the previous brush.
    pub fn apply_brush(&self, brush: &Brush) -> Brush {
        let hbrush = unsafe { SelectObject(self.hdc, brush.hbrush as HGDIOBJ) } as HBRUSH;
        Brush { hbrush }
    }
    /// Apply a font for the graphics context.
    /// Return a Font object that can be used to restore the previous font.
    pub fn apply_font(&self, font: &Font) -> Font {
        let hfont = unsafe { SelectObject(self.hdc, font.hfont as HGDIOBJ) } as HFONT;
        Font { hfont }
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

    /// Set the text alignment for the graphics context.
    /// It will affect the position of the reference point for text drawing.
    pub fn set_textalign(&self, align: TextAligns) {
        unsafe {
            SetTextAlign(self.hdc, align);
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

    /// Draw a pie with the given arguments.
    /// Its style is determined by the current pen.
    /// Parameter `pos` is the center of the pie,
    /// `xr` and `yr` are the x direction and y direction radii of the pie,
    /// `start` and `end` are the start and end angles of the pie in radians.
    /// The angles are measured from the x-axis(horizontal direction) in a counter-clockwise direction.
    pub fn pie(&self, pos: Point, xr: i32, yr: i32, start: f64, end: f64) {
        unsafe {
            MoveToEx(self.hdc, pos.x, pos.y, null_mut());
            ArcTo(
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
            LineTo(self.hdc, pos.x, pos.y);
        }
    }

    /// Draw an ellipse with the given arguments.
    /// Its style is determined by the current pen.
    /// Parameter `pos` is the center of the ellipse,
    /// `xr` and `yr` are the x direction and y direction radii of the ellipse.
    pub fn ellipse(&self, pos: Point, xr: i32, yr: i32) {
        self.arc(pos, xr, yr, 0.0, 0.0);
    }

    /// Draw a circle with the given arguments.
    /// Its style is determined by the current pen.
    /// Parameter `pos` is the center of the circle,
    /// `r` is the radius of the circle.
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

    /// Draw a filled pie with the given arguments.
    /// Its style is determined by the current brush.
    /// Parameter `pos` is the center of the pie,
    /// `xr` and `yr` are the x direction and y direction radii of the pie,
    /// `start` and `end` are the start and end angles of the pie in radians.
    /// The angles are measured from the x-axis(horizontal direction) in a counter-clockwise direction.
    pub fn fillpie(&self, pos: Point, xr: i32, yr: i32, start: f64, end: f64) {
        unsafe {
            Pie(
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

    /// Draw a filled ellipse with the given arguments.
    /// Its style is determined by the current brush.
    /// Parameter `pos` is the center of the ellipse,
    /// `xr` and `yr` are the x direction and y direction radii of the ellipse.
    pub fn fillellipse(&self, pos: Point, xr: i32, yr: i32) {
        unsafe {
            Ellipse(self.hdc, pos.x - xr, pos.y - yr, pos.x + xr, pos.y + yr);
        }
    }

    /// Draw a filled circle with the given arguments.
    /// Its style is determined by the current brush.
    /// Parameter `pos` is the center of the circle,
    /// `r` is the radius of the circle.
    pub fn fillcircle(&self, pos: Point, r: i32) {
        self.fillellipse(pos, r, r);
    }

    /// Draw text at the given position.
    /// Its font and color are determined by the current font and text color.
    pub fn xytext(&self, text: &str, p: Point) {
        let text = text
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        unsafe {
            TextOutW(
                self.hdc,
                p.x,
                p.y,
                text.as_ptr() as _,
                (text.len() - 1) as _,
            );
        }
    }

    /// Draw text within the given rectangle.
    /// Its font and color are determined by the current font and text color.
    /// The text will be aligned according to the given format.
    pub fn recttext(&self, rect: Rect, text: &str, format: TextFormats) {
        let text = text
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let len = text.len() - 1;
        let mut rect = RECT {
            left: rect.pos.x,
            top: rect.pos.y,
            right: rect.pos.x + rect.size.width,
            bottom: rect.pos.y + rect.size.height,
        };
        unsafe {
            DrawTextW(self.hdc, text.as_ptr() as _, len as i32, &mut rect, format);
        }
    }
}
