use crate::Point;

/// The enum of Button
/// The point coordinates in the enumeration are used to indicate the mouse coordinates at the time of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left(Point),   // Left mouse button with the point coordinates
    Right(Point),  // Right mouse button with the point coordinates
    Middle(Point), // Middle mouse button with the point coordinates
}

/// The enum of Wheel
/// The i16 value in the enumeration is used to indicate the number of clicks of the mouse wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wheel {
    Up(i16),   // Mouse wheel up with the number of clicks
    Down(i16), // Mouse wheel down with the number of clicks
    None,      // No mouse wheel event
}

/// The enum of directional keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    Left,  // Left arrow key
    Right, // Right arrow key
    Up,    // Up arrow key
    Down,  // Down arrow key
}

/// The enum of keys
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

/// The enum of window size events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeType {
    Maximized, // Window is maximized
    Minimized, // Window is minimized
    Restored,  // Window is restored from minimized or maximized state
    MaxHide,   // Window is maximized and hidden from taskbar
    MaxShow,   // Window is maximized and shown in taskbar

    Unknown, // Unknown size type
}
