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

/// A macro to create a `Point` struct.
#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr $(,)?) => {
        $crate::Point { x: $x, y: $y }
    };
}

/// A macro to create a `Size` struct.
#[macro_export]
macro_rules! size {
    ($w:expr, $h:expr $(,)?) => {
        $crate::Size {
            width: $w,
            height: $h,
        }
    };
}

/// A macro to create a `Rect` struct.
#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr $(,)?) => {
        $crate::Rect {
            pos: $crate::pos!($x, $y),
            size: $crate::size!($w, $h),
        }
    };
}

use std::ops::{Add, AddAssign, Sub, SubAssign};

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

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl SubAssign for Size {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

use std::ops::{Div, DivAssign, Mul, MulAssign};

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

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}
impl DivAssign<f32> for Point {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}
impl DivAssign<f32> for Size {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}
impl MulAssign<f32> for Size {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Point {
    pub fn distance(self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn within(&self, rect: &Rect) -> bool {
        self.x >= rect.pos.x
            && self.x < rect.pos.x + rect.size.width
            && self.y >= rect.pos.y
            && self.y < rect.pos.y + rect.size.height
    }
}

impl Size {
    pub fn area(self) -> i32 {
        self.width * self.height
    }
}

impl Rect {
    /// If another rectangle is completely contained by it, return true.
    pub fn contains(&self, other: &Rect) -> bool {
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

    /// Return the center point of the rectangle.
    pub fn center(&self) -> Point {
        pos!(
            self.pos.x + self.size.width / 2,
            self.pos.y + self.size.height / 2,
        )
    }

    /// Return a new rectangle with the given size and centered around the center of the original rectangle.
    pub fn center_rect(&self, size: Size) -> Rect {
        let center = self.center();
        rect!(
            center.x - size.width / 2,
            center.y - size.height / 2,
            size.width,
            size.height,
        )
    }

    pub fn top(&self) -> i32 {
        self.pos.y
    }
    pub fn bottom(&self) -> i32 {
        self.pos.y + self.size.height
    }
    pub fn left(&self) -> i32 {
        self.pos.x
    }
    pub fn right(&self) -> i32 {
        self.pos.x + self.size.width
    }

    pub fn top_left(&self) -> Point {
        pos!(self.pos.x, self.pos.y)
    }
    pub fn top_right(&self) -> Point {
        pos!(self.pos.x + self.size.width, self.pos.y)
    }
    pub fn bottom_left(&self) -> Point {
        pos!(self.pos.x, self.pos.y + self.size.height)
    }
    pub fn bottom_right(&self) -> Point {
        pos!(self.pos.x + self.size.width, self.pos.y + self.size.height,)
    }
}

use std::ops::{BitAnd, BitOr};

impl BitAnd for Rect {
    type Output = Option<Rect>;
    /// Return the intersection of two rectangles, or None if they don't intersect.
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

impl Into<(i32, i32, i32, i32)> for Rect {
    fn into(self) -> (i32, i32, i32, i32) {
        (self.pos.x, self.pos.y, self.size.width, self.size.height)
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        rect!(value.0, value.1, value.2, value.3)
    }
}

impl Into<(i32, i32)> for Size {
    fn into(self) -> (i32, i32) {
        (self.width, self.height)
    }
}

impl From<(i32, i32)> for Size {
    fn from(value: (i32, i32)) -> Self {
        size!(value.0, value.1)
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        pos!(value.0, value.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// A macro to create a `Color` struct.
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
    ($gray: expr $(,)?) => {
        $crate::Color {
            red: $gray,
            green: $gray,
            blue: $gray,
            alpha: 255,
        }
    };
}

impl Color {
    pub const BLACK: Color = rgb!(0);
    pub const WHITE: Color = rgb!(255, 255, 255);
    pub const RED: Color = rgb!(255, 0, 0);
    pub const GREEN: Color = rgb!(0, 255, 0);
    pub const BLUE: Color = rgb!(0, 0, 255);
    pub const YELLOW: Color = rgb!(255, 255, 0);
    pub const CYAN: Color = rgb!(0, 255, 255);
    pub const MAGENTA: Color = rgb!(255, 0, 255);
    pub const TRANSPARENT: Color = rgb!(0, 0, 0, 0);
    pub const GRAY: Color = rgb!(160);
    pub const LIGHT_GRAY: Color = rgb!(210);
    pub const DARK_GRAY: Color = rgb!(85);
}
