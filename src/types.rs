#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: $a,
        }
    };
}

#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        rgba!($r, $g, $b, 255)
    };
}

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
