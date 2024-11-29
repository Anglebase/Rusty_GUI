/// This file contains definitions for some extension types.
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------

use std::ops::{Add, Sub, Mul, Div};
use crate::{Point, Size};

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: (self.x as f32 * other) as i32,
            y: (self.y as f32 * other) as i32,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: (self.x as f32 / other) as i32,
            y: (self.y as f32 / other) as i32,
        }
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            width: (self.width as f32 * other) as i32,
            height: (self.height as f32 * other) as i32,
        }
    }
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            width: (self.width as f32 / other) as i32,
            height: (self.height as f32 / other) as i32,
        }
    }
}