//! This file contains the implementation of the Canvas struct and its dependent structs.
//! The `Canvas` struct is used to draw shapes and text on the screen.

use std::os::raw::c_void;

use crate::*;

/// The `LineStyle` is used to specify the style of the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,      // Solid line
    Dash,       // Dashed line
    Dot,        // Dotted line
    DashDot,    // Dashed-dotted line
    DashDotDot, // Dashed-dot-dotted line
    Null,       // Empty line(It will not be drawn)
}

/// The `JoinStyle` is used to specify the style of the join between two lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinStyle {
    Miter, // Miter join
    Round, // Round join
    Bevel, // Bevel join
}

/// The `CapStyle` is used to specify the style of the end caps of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapStyle {
    Flat,   // Flat cap
    Square, // Square cap
    Round,  // Round cap
}

/// The `PenStyle` is used to specify the style of the pen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PenStyle {
    pub line_style: LineStyle, // Line style
    pub width: u32,            // Line width
    pub color: Color,          // Line color
    pub join_style: JoinStyle, // Join style
    pub cap_style: CapStyle,   // Cap style
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

/// The `Pen` is used to specify the style of the pen.
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
    /// Create a new `Pen` with `style`.
    pub fn new(style: PenStyle) -> Self {
        Self {
            hpen: new_pen_object(style),
        }
    }
}

/// The `Brush` is used to specify the color of the brush.
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
    /// Create a new `Brush` with `color`.
    pub fn new(color: Color) -> Self {
        Self {
            hbrush: new_brush_object(BrushParam::Solid(color)),
        }
    }
}

/// The `Font` is used to specify the font.
#[derive(Clone)]
pub struct Font {
    pub(crate) hfont: *mut c_void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Default = 0,     // Default weight
    Thin = 100,       // Thin weight
    ExtraLight = 200, // Extra light weight
    Light = 300,      // Light weight
    Normal = 400,     // Normal weight
    Medium = 500,     // Medium weight
    SemiBold = 600,   // Semi-bold weight
    Bold = 700,       // Bold weight
    ExtraBold = 800,  // Extra bold weight
    Black = 900,      // Black weight
}

/// The `FontStyle` is used to specify the font style.
#[derive(Clone)]
pub struct FontStyle {
    pub size: i32,          // Font size
    pub weight: FontWeight, // Font weight
    pub italic: bool,       // Italic
    pub underline: bool,    // Underline
    pub strikeout: bool,    // Strikeout
    pub font: String,       // Font name
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
    /// Create a new `Font` with `style`.
    pub fn new(style: FontStyle) -> Self {
        let hfont = new_font_object(style);
        Self { hfont }
    }
}

/// The `TextAlign` is used to specify the alignment of the text.
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

/// The `Canvas` is used to draw shapes and text on the screen.
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
    /// It uses the current pen.
    pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        draw_line(self.hdc, x1, y1, x2, y2);
    }

    /// Draw a rectangle with `rect`.
    /// It uses the current pen.
    pub fn rect(&self, rect: Rect) {
        draw_rect(self.hdc, rect);
    }

    /// Draw a rounded rectangle with `rect` and `rx` and `ry`.
    /// It uses the current pen.
    pub fn round_rect(&self, rect: Rect, rx: i32, ry: i32) {
        draw_round_rect(self.hdc, rect, rx, ry);
    }

    /// Draw a polygon with `points`.
    /// It uses the current pen.
    pub fn polyline(&self, points: &[Point]) {
        draw_polyline(self.hdc, points);
    }

    /// Draw a polygon with `points`.
    /// It uses the current pen.
    pub fn polygon(&self, points: &[Point]) {
        draw_polygon(self.hdc, points);
    }

    /// Draw an arc with `rect`, `start` and `sweep`.
    /// It uses the current pen.
    pub fn arc(&self, rect: Rect, start: f32, sweep: f32) {
        draw_arc(self.hdc, rect, start, sweep);
    }

    /// Draw a pie with `rect`, `start` and `sweep`.
    /// It uses the current pen.
    pub fn pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_pie(self.hdc, rect, start, sweep);
    }

    /// Draw an ellipse with `rect`.
    /// It uses the current pen.
    pub fn ellipse(&self, rect: Rect) {
        draw_ellipse(self.hdc, rect);
    }

    /// Draw a circle with `pos` and `radius`.
    /// It uses the current pen.
    pub fn circle(&self, pos: Point, radius: i32) {
        draw_circle(self.hdc, pos, radius);
    }

    /// Draw a fill rectangle with `rect`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_rect(&self, rect: Rect) {
        draw_fill_rect(self.hdc, rect);
    }

    /// Draw a fill rounded rectangle with `rect` and `rx` and `ry`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_round_rect(&self, rect: Rect, rx: i32, ry: i32) {
        draw_fill_round_rect(self.hdc, rect, rx, ry);
    }

    /// Draw a fill polygon with `points`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_polygon(&self, points: &[Point]) {
        draw_fill_polygon(self.hdc, points);
    }

    /// Draw a fill pie with `rect`, `start` and `sweep`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_fill_pie(self.hdc, rect, start, sweep);
    }

    /// Draw a fill ellipse with `rect`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_ellipse(&self, rect: Rect) {
        draw_fill_ellipse(self.hdc, rect);
    }

    /// Draw a fill circle with `pos` and `radius`.
    /// It uses the current pen for outline and brush for fill.
    pub fn fill_circle(&self, pos: Point, radius: i32) {
        draw_fill_circle(self.hdc, pos, radius);
    }

    /// Draw a text with `pos` and `text`.
    /// It uses the current pen and font.
    pub fn xy_text(&self, pos: Point, text: &str, align: TextAlign) {
        draw_xy_text(self.hdc, pos, text, align);
    }

    /// Draw a text with `rect` and `text`.
    /// It uses the current pen and font.
    pub fn rect_text(&self, rect: Rect, text: &str, align: TextAlign) {
        draw_rect_text(self.hdc, rect, text, align);
    }
}
