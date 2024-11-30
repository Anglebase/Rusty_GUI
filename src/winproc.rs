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

/// The `WinProc` trait defines the interface of window procedures.
/// If you want to create a custom window, you should implement this trait for your window struct.
/// Then create the window by the `Window::new(...)`.
#[allow(unused)]
pub trait WinProc {
    /// This method is called when the window is created.
    /// You can add initialization code here.
    fn init(&mut self) {}
    /// This method is called when the window is destroyed.
    /// You can add cleanup code here.
    fn destroy(&mut self) {}

    /// This method is called when the window needs to be redrawn.
    fn draw(&mut self, g: &mut Graphics) {}

    /// This method is called when a mouse button is pressed.
    fn button_down(&mut self, button: Button) {}
    /// This method is called when a mouse button is released.
    fn button_up(&mut self, button: Button) {}
    /// This method is called when a mouse button is double-clicked.
    fn button_dbclk(&mut self, button: Button) {}
    /// This method is called when the mouse is moved.
    fn mouse_move(&mut self, point: Point, key: Option<Key>) {}
    /// This method is called when the mouse wheel is scrolled.
    fn mouse_wheel(&mut self, point: Point, wheel: Wheel, key: Option<Key>) {}

    /// This method is called when a key is pressed.
    fn key_down(&mut self, key: Key) {}
    /// This method is called when a key is released.
    fn key_up(&mut self, key: Key) {}
    /// This method is called when a character is input.
    fn input(&mut self, text: &str) {}

    /// This method is called when the window is resized.
    fn window_resize(&mut self, size: Size, size_type: SizeType) {}
    /// This method is called when the window is moved.
    fn window_move(&mut self, point: Point) {}
}
