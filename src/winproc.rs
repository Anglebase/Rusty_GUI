/// This file defines traits related to window procedures as interfaces.
/// author: Anglebase (https://github.com/Anglebase)
/// --------------------------------------------------------------------
use crate::Graphics;
use crate::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left(Point),
    Right(Point),
    Middle(Point),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wheel {
    Up(i16),
    Down(i16),
    None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Alpha(char), // A-Z
    N(char),     // 0-9
    F(u8),       // F1-F12
    Num(u8),     // NumPad 0-9

    Shift, // Shift
    Ctrl,  // Ctrl
    Alt,   // Alt

    // Special symbols for American keyboards, such as: ` , . / ; ' [ ] \ - =
    Symbol(char), // (Here use the symbol output when shift is not pressed to represent)

    NumAdd, // + on NumPad
    NumSub, // - on NumPad
    NumMul, // * on NumPad
    NumDiv, // / on NumPad
    NumDot, // . on NumPad

    Tab,       // Tab
    Space,     // Space
    Enter,     // Enter
    Backspace, // Backspace

    Esc,        // Esc
    CapsLock,   // CapsLock
    LeftCtrl,   // Left Ctrl
    LeftShift,  // Left Shift
    LeftAlt,    // Left Alt
    RightCtrl,  // Right Ctrl
    RightShift, // Right Shift
    RightAlt,   // Right Alt
    ScrollLock, // ScrollLock
    NumLock,    // NumLock
    Delete,     // Delete(Del)
    Insert,     // Insert(Ins)
    Home,       // Home
    End,        // End
    PageUp,     // PageUp(PgUp)
    PageDown,   // PageDown(PgDn)

    LeftButton,   // Left mouse button
    RightButton,  // Right mouse button
    MiddleButton, // Middle mouse button
    X1Button,     // mouse extension button 1
    X2Button,     // mouse extension button 2

    Arrow(ArrowType), // Arrow keys

    Unknown(i32), // Other keys
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeType {
    Maximized,
    Minimized,
    Restored,
    MaxHide,
    MaxShow,

    Unknown,
}

#[allow(unused)]
pub trait WinProc {
    fn draw(&mut self, g: &mut Graphics) {}

    fn button_down(&mut self, button: Button) {}
    fn button_up(&mut self, button: Button) {}
    fn button_dbclk(&mut self, button: Button) {}
    fn mouse_move(&mut self, point: Point, key: Option<Key>) {}
    fn mouse_wheel(&mut self, point: Point, wheel: Wheel, key: Option<Key>) {}

    fn key_down(&mut self, key: Key) {}
    fn key_up(&mut self, key: Key) {}
    fn input(&mut self, text: &str) {}

    fn window_resize(&mut self, size: Size, size_type: SizeType) {}
    fn window_move(&mut self, point: Point) {}
}
