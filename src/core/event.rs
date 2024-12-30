//! This file is used to define the Event enum and its variants.

use crate::HotKeyFlags;
use crate::Point;
use crate::Size;

/// The Event enum represents all possible events that can occur in a GUI application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    KeyPressed {
        key: KeyCode,
        sys: bool,
    },
    KeyReleased {
        key: KeyCode,
        sys: bool,
    },
    MouseMoved {
        pos: Point,
        mk: ModifierKey,
    },
    MouseButtonPressed {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },
    MouseButtonReleased {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },
    MouseWheelScrolled {
        wheel: MouseWheel,
        pos: Point,
        mk: ModifierKey,
    },
    MouseDoubleClicked {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },

    WindowCreated,
    WindowDestroyed,
    WindowMoved {
        pos: Point,
    },
    WindowDisable,
    WindowEnable,

    WindowResized {
        size: Size,
        ty: WindowSize,
    },

    Input {
        ch: char,
    },
    HotKey {
        key: KeyCode,
        modifiers: HotKeyFlags,
    },

    Timer {
        id: usize,
    },

    Hover {
        pos: Point,
        mk: ModifierKey,
    },
    Leave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
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
    Clear,      // Clear(Num 5)

    LeftButton,   // Left mouse button
    RightButton,  // Right mouse button
    MiddleButton, // Middle mouse button
    X1Button,     // mouse extension button 1
    X2Button,     // mouse extension button 2

    Left,  // Left arrow
    Right, // Right arrow
    Up,    // Up arrow
    Down,  // Down arrow

    Unknown(i32), // Other keys
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseWheel {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierKey {
    Shift,
    Ctrl,
    Alt,
    Win,
    Mouse(MouseButton),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSize {
    Resize,
    Minimize,
    Maximize,
    Restore,
    MaxHide,
    MaxShow,
}
