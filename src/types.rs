#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr $(,)?) => {
        $crate::Point { x: $x, y: $y }
    };
}
#[macro_export]
macro_rules! size {
    ($w:expr, $h:expr $(,)?) => {
        $crate::Size {
            width: $w,
            height: $h,
        }
    };
}
#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr $(,)?) => {
        $crate::Rect {
            pos: $crate::pos!($x, $y),
            size: $crate::size!($w, $h),
        }
    };
}

use std::ops::{Add, Sub};

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        pos!(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        pos!(self.x - other.x, self.y - other.y)
    }
}

impl Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        size!(self.width + other.width, self.height + other.height)
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        size!(self.width - other.width, self.height - other.height)
    }
}

use std::ops::{Div, Mul};

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        let x = (self.x as f32 * other) as i32;
        let y = (self.y as f32 * other) as i32;
        pos!(x, y)
    }
}

impl Div<f32> for Point {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let x = (self.x as f32 / other) as i32;
        let y = (self.y as f32 / other) as i32;
        pos!(x, y)
    }
}

impl Mul<f32> for Size {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        let w = (self.width as f32 * other) as i32;
        let h = (self.height as f32 * other) as i32;
        size!(w, h)
    }
}

impl Div<f32> for Size {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let w = (self.width as f32 / other) as i32;
        let h = (self.height as f32 / other) as i32;
        size!(w, h)
    }
}

impl Point {
    pub fn distance(self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

impl Rect {
    pub fn contanins(&self, other: &Rect) -> bool {
        if self.size.width < other.size.width || self.size.height < other.size.height {
            return false;
        }

        let this_left = self.pos.x;
        let this_top = self.pos.y;
        let this_right = this_left + self.size.width;
        let this_bottom = this_top + self.size.height;

        let other_left = other.pos.x;
        let other_top = other.pos.y;
        let other_right = other_left + other.size.width;
        let other_bottom = other_top + other.size.height;

        this_left <= other_left
            && this_right >= other_right
            && this_top <= other_top
            && this_bottom >= other_bottom
    }
}

use std::ops::{BitAnd, BitOr};

impl BitAnd for Rect {
    type Output = Option<Rect>;
    fn bitand(self, other: Self) -> Option<Rect> {
        let this_left = self.pos.x;
        let this_top = self.pos.y;
        let this_right = this_left + self.size.width;
        let this_bottom = this_top + self.size.height;

        let other_left = other.pos.x;
        let other_top = other.pos.y;
        let other_right = other_left + other.size.width;
        let other_bottom = other_top + other.size.height;

        let new_left = i32::max(this_left, other_left);
        let new_top = i32::max(this_top, other_top);
        let new_right = i32::min(this_right, other_right);
        let new_bottom = i32::min(this_bottom, other_bottom);

        let new_rect = rect!(
            new_left,
            new_top,
            new_right - new_left,
            new_bottom - new_top,
        );

        if new_rect.size.width > 0 && new_rect.size.height > 0 {
            Some(new_rect)
        } else {
            None
        }
    }
}

impl BitOr for Rect {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        let this_left = self.pos.x;
        let this_top = self.pos.y;
        let this_right = this_left + self.size.width;
        let this_bottom = this_top + self.size.height;

        let other_left = other.pos.x;
        let other_top = other.pos.y;
        let other_right = other_left + other.size.width;
        let other_bottom = other_top + other.size.height;

        let new_left = i32::min(this_left, other_left);
        let new_top = i32::min(this_top, other_top);
        let new_right = i32::max(this_right, other_right);
        let new_bottom = i32::max(this_bottom, other_bottom);

        rect!(
            new_left,
            new_top,
            new_right - new_left,
            new_bottom - new_top,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr $(,)?) => {
        $crate::Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: 255,
        }
    };
    ($r:expr, $g:expr, $b:expr, $a:expr $(,)?) => {
        $crate::Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: $a,
        }
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
    pub const TRANSPARENT: Color = rgb!(0, 0, 0, 0);
}