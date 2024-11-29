/// This file contains system API interactions and encapsulation based on device handles.
/// author: Anglebase (https://github.com/Anglebase)
/// -------------------------------------------------------------------------------------

use crate::Graphics;
use winapi::{
    shared::windef::*,
    um::{wingdi::*, winuser::*},
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    #[allow(unused)]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    #[allow(unused)]
    pub fn red(&self) -> u8 {
        self.red
    }
    #[allow(unused)]
    pub fn green(&self) -> u8 {
        self.green
    }
    #[allow(unused)]
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

impl Graphics {
    pub fn full_clear(&mut self, color: Color) {
        unsafe {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetClientRect(self.hwnd, &mut rect);
            let bs = CreateSolidBrush(RGB(color.red(), color.green(), color.blue()));
            FillRect(self.hdc, &rect, bs);
            DeleteObject(bs as HGDIOBJ);
        }
    }
}
