use std::os::raw::c_void;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinStyle {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapStyle {
    Flat,
    Square,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PenStyle {
    pub line_style: LineStyle,
    pub width: u32,
    pub color: Color,
    pub join_style: JoinStyle,
    pub cap_style: CapStyle,
}

impl Default for PenStyle {
    fn default() -> Self {
        Self {
            line_style: LineStyle::Solid,
            width: 1,
            color: Color::BLACK,
            join_style: JoinStyle::Miter,
            cap_style: CapStyle::Flat,
        }
    }
}

#[derive(Clone)]
pub struct Pen {
    hpen: *mut c_void,
}
impl Drop for Pen {
    fn drop(&mut self) {
        delete_object(self.hpen);
    }
}
impl Pen {
    pub fn new(style: PenStyle) -> Self {
        Self {
            hpen: new_pen_object(style),
        }
    }
}

#[derive(Clone)]
pub struct Brush {
    hbrush: *mut c_void,
}
impl Drop for Brush {
    fn drop(&mut self) {
        delete_object(self.hbrush);
    }
}
impl Brush {
    pub fn new(color: Color) -> Self {
        Self {
            hbrush: new_brush_object(BrushParam::Solid(color)),
        }
    }
}

#[derive(Clone)]
pub struct Font {
    pub(crate) hfont: *mut c_void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Dontcare = 0,
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[derive(Clone)]
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
        delete_object(self.hfont);
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
            font: String::from("Arial"),
        }
    }
}

impl Font {
    pub fn new(style: FontStyle) -> Self {
        let hfont = new_font_object(style);
        Self { hfont }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    LeftTop,
    LeftMiddle,
    LeftBottom,
    CenterTop,
    Center,
    CenterBottom,
    RightTop,
    RightMiddle,
    RightBottom,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::LeftTop
    }
}

pub struct Canvas {
    pub(crate) hdc: *mut c_void,
    pub(crate) rect: Rect,
}

impl Canvas {
    /// Clear current window content with `color`.
    pub fn clear(&self, color: Color) {
        clear_device(self.hdc, self.rect, color);
    }

    /// Set current pen to `pen`.
    pub fn set_pen(&self, pen: &Pen) -> Pen {
        Pen {
            hpen: select_object(self.hdc, pen.hpen) as *mut c_void,
        }
    }

    /// Set current brush to `brush`.
    pub fn set_brush(&self, brush: &Brush) -> Brush {
        Brush {
            hbrush: select_object(self.hdc, brush.hbrush) as *mut c_void,
        }
    }

    /// Set current font to `font`.
    pub fn set_font(&self, font: &Font) -> Font {
        Font {
            hfont: select_object(self.hdc, font.hfont) as *mut c_void,
        }
    }

    /// Draw a line from `(x1, y1)` to `(x2, y2)`.
    pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        draw_line(self.hdc, x1, y1, x2, y2);
    }

    /// Draw a rectangle with `rect`.
    pub fn rect(&self, rect: Rect) {
        draw_rect(self.hdc, rect);
    }

    /// Draw a rounded rectangle with `rect` and `rx` and `ry`.
    pub fn round_rect(&self, rect: Rect, rx: i32, ry: i32) {
        draw_round_rect(self.hdc, rect, rx, ry);
    }

    /// Draw a polygon with `points`.
    pub fn polyline(&self, points: &[Point]) {
        draw_polyline(self.hdc, points);
    }

    /// Draw a polygon with `points`.
    pub fn polygon(&self, points: &[Point]) {
        draw_polygon(self.hdc, points);
    }

    /// Draw an arc with `rect`, `start` and `sweep`.
    pub fn arc(&self, rect: Rect, start: f32, sweep: f32) {
        draw_arc(self.hdc, rect, start, sweep);
    }

    /// Draw a pie with `rect`, `start` and `sweep`.
    pub fn pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_pie(self.hdc, rect, start, sweep);
    }

    /// Draw an ellipse with `rect`.
    pub fn ellipse(&self, rect: Rect) {
        draw_ellipse(self.hdc, rect);
    }

    /// Draw a circle with `pos` and `radius`.
    pub fn circle(&self, pos: Point, radius: i32) {
        draw_circle(self.hdc, pos, radius);
    }

    /// Draw a fill rectangle with `rect`.
    pub fn fill_rect(&self, rect: Rect) {
        draw_fill_rect(self.hdc, rect);
    }

    /// Draw a fill rounded rectangle with `rect` and `rx` and `ry`.
    pub fn fill_round_rect(&self, rect: Rect, rx: i32, ry: i32) {
        draw_fill_round_rect(self.hdc, rect, rx, ry);
    }

    /// Draw a fill polygon with `points`.
    pub fn fill_polygon(&self, points: &[Point]) {
        draw_fill_polygon(self.hdc, points);
    }

    /// Draw a fill pie with `rect`, `start` and `sweep`.
    pub fn fill_pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_fille_pie(self.hdc, rect, start, sweep);
    }

    /// Draw a fill ellipse with `rect`.
    pub fn fill_ellipse(&self, rect: Rect) {
        draw_fille_ellipse(self.hdc, rect);
    }

    /// Draw a fill circle with `pos` and `radius`.
    pub fn fill_circle(&self, pos: Point, radius: i32) {
        draw_fille_circle(self.hdc, pos, radius);
    }

    /// Draw a text with `pos` and `text`.
    pub fn xytext(&self, pos: Point, text: &str, align: TextAlign) {
        draw_xy_text(self.hdc, pos, text, align);
    }

    /// Draw a text with `rect` and `text`.
    pub fn rect_text(&self, rect: Rect, text: &str, align: TextAlign) {
        draw_rect_text(self.hdc, rect, text, align);
    }
}
