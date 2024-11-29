/// This file defines traits related to window procedures as interfaces.
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------------------
use crate::Graphics;

#[allow(unused)]
pub trait WinProc {
    fn draw(&self, g: &mut Graphics) {}
    fn left_button_down(&self, x: i32, y: i32) {}
}
