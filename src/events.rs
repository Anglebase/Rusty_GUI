use crate::Point;

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
    Maximized, // Window is maximized
    Minimized, // Window is minimized
    Restored,  // Window is restored from minimized or maximized state
    MaxHide,   // Window is maximized and hidden from taskbar
    MaxShow,   // Window is maximized and shown in taskbar

    Unknown, // Unknown size type
}
