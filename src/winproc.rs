/// This file defines traits related to window procedures as interfaces.
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------------------

use crate::Graphics;

#[allow(unused)]
pub trait WinProc {
    fn draw(&self, graphics: &mut Graphics) {
    }
    fn other(&self) {
    }
}