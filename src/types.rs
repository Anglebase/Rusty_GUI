/// A 2D point with `x` and `y` coordinates
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A 2D size with `width` and `height` dimensions
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// A common rectangle with a position and a size
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

/// A color with `red`, `green`, `blue`, and `alpha` components
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// A macro to create a Point
#[macro_export]
macro_rules! p {
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

/// A macro to create a Size
#[macro_export]
macro_rules! s {
    ($w:expr, $h:expr) => {
        Size { width: $w, height: $h }
    };
}

/// A macro to create a Rect
#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect {
            pos: p!($x, $y),
            size: s!($w, $h),
        }
    };
}

/// A macro to create a Color by specifying its RGB values and an optional alpha value (default is 255)
#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: $a,
        }
    };
    ($r:expr, $g:expr, $b:expr) => {
        rgb!($r, $g, $b, 255)
    };
}

/// A set of predefined colors
impl Color {
    pub const BLACK: Color = rgb!(0, 0, 0);
    pub const WHITE: Color = rgb!(255, 255, 255);
    pub const RED: Color = rgb!(255, 0, 0);
    pub const GREEN: Color = rgb!(0, 255, 0);
    pub const BLUE: Color = rgb!(0, 0, 255);
    pub const YELLOW: Color = rgb!(255, 255, 0);
    pub const CYAN: Color = rgb!(0, 255, 255);
    pub const MAGENTA: Color = rgb!(255, 0, 255);
}

impl Rect {
    /// Returns the center point coordinate of the rectangle
    pub fn center(&self) -> Point {
        Point {
            x: self.pos.x + self.size.width / 2,
            y: self.pos.y + self.size.height / 2,
        }
    }
}