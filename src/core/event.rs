//! This file is used to define the Event enum and its variants.

use crate::HotKeyFlags;
use crate::Point;
use crate::Size;

/// The Event enum represents all possible events that can occur in a GUI application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A key has been pressed.
    /// - `key`: The specific key code that was pressed.
    /// - `sys`: A boolean indicating if the key is a system key.
    KeyPressed {
        key: KeyCode,
        sys: bool,
    },

    /// A key has been released.
    /// - `key`: The specific key code that was released.
    /// - `sys`: A boolean indicating if the key is a system key.
    KeyReleased {
        key: KeyCode,
        sys: bool,
    },

    /// The mouse cursor has moved.
    /// - `pos`: The new position of the mouse cursor.
    /// - `mk`: The modifier key state when the mouse moved.
    MouseMoved {
        pos: Point,
        mk: ModifierKey,
    },

    /// A mouse button has been pressed.
    /// - `button`: The specific mouse button that was pressed.
    /// - `pos`: The position of the mouse cursor when the button was pressed.
    /// - `mk`: The modifier key state when the button was pressed.
    MouseButtonPressed {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },

    /// A mouse button has been released.
    /// - `button`: The specific mouse button that was released.
    /// - `pos`: The position of the mouse cursor when the button was released.
    /// - `mk`: The modifier key state when the button was released.
    MouseButtonReleased {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },

    /// The mouse wheel has been scrolled.
    /// - `wheel`: The direction of the scroll.
    /// - `pos`: The position of the mouse cursor when the scroll occurred.
    /// - `mk`: The modifier key state when the scroll occurred.
    MouseWheelScrolled {
        wheel: MouseWheel,
        pos: Point,
        mk: ModifierKey,
    },

    /// A mouse button has been double-clicked.
    /// - `button`: The specific mouse button that was double-clicked.
    /// - `pos`: The position of the mouse cursor when the double-click occurred.
    /// - `mk`: The modifier key state when the double-click occurred.
    MouseDoubleClicked {
        button: MouseButton,
        pos: Point,
        mk: ModifierKey,
    },

    /// A window has been created.
    WindowCreated,

    /// A window has been destroyed.
    WindowDestroyed,

    /// A window has been moved.
    /// - `pos`: The new position of the window.
    WindowMoved {
        pos: Point,
    },

    /// A window has been disabled.
    WindowDisable,

    /// A window has been enabled.
    WindowEnable,

    /// A window has been resized.
    /// - `size`: The new size of the window.
    /// - `ty`: The type of resize operation.
    WindowResized {
        size: Size,
        ty: WindowSize,
    },

    /// Input has been received.
    /// - `ch`: The character input.
    Input {
        ch: char,
    },

    /// A hot key has been pressed.
    /// - `key`: The specific key code that was pressed.
    /// - `modifiers`: The hot key flags representing the modifier keys.
    HotKey {
        key: KeyCode,
        modifiers: HotKeyFlags,
    },

    /// A timer event has occurred.
    /// - `id`: The identifier for the timer.
    Timer {
        id: usize,
    },

    /// The mouse cursor has hovered over an element.
    /// - `pos`: The position of the mouse cursor when hovering.
    /// - `mk`: The modifier key state when hovering.
    Hover {
        pos: Point,
        mk: ModifierKey,
    },

    /// The mouse cursor has left an element.
    Leave,
}

/// Represents the different key codes that can be used in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    /// Represents alphabetic keys A-Z.
    Alpha(char),
    /// Represents numeric keys 0-9.
    N(char),
    /// Represents function keys F1-F12.
    F(u8),
    /// Represents numeric keypad keys 0-9.
    Num(u8),

    /// Represents the Shift key.
    Shift,
    /// Represents the Ctrl key.
    Ctrl,
    /// Represents the Alt key.
    Alt,

    /// Represents special symbols on the keyboard, such as: ` , . / ; ' [ ] \ - =.
    /// Here, the symbol output when the Shift key is not pressed is used to represent.
    Symbol(char),

    /// Represents the + key on the numeric keypad.
    NumAdd,
    /// Represents the - key on the numeric keypad.
    NumSub,
    /// Represents the * key on the numeric keypad.
    NumMul,
    /// Represents the / key on the numeric keypad.
    NumDiv,
    /// Represents the . key on the numeric keypad.
    NumDot,

    /// Represents the Tab key.
    Tab,
    /// Represents the Space key.
    Space,
    /// Represents the Enter key.
    Enter,
    /// Represents the Backspace key.
    Backspace,

    /// Represents the Esc key.
    Esc,
    /// Represents the CapsLock key.
    CapsLock,
    /// Represents the Left Ctrl key.
    LeftCtrl,
    /// Represents the Left Shift key.
    LeftShift,
    /// Represents the Left Alt key.
    LeftAlt,
    /// Represents the Right Ctrl key.
    RightCtrl,
    /// Represents the Right Shift key.
    RightShift,
    /// Represents the Right Alt key.
    RightAlt,
    /// Represents the ScrollLock key.
    ScrollLock,
    /// Represents the NumLock key.
    NumLock,
    /// Represents the Delete key (Del).
    Delete,
    /// Represents the Insert key (Ins).
    Insert,
    /// Represents the Home key.
    Home,
    /// Represents the End key.
    End,
    /// Represents the PageUp key (PgUp).
    PageUp,
    /// Represents the PageDown key (PgDn).
    PageDown,
    /// Represents the Clear key (Num 5).
    Clear,

    /// Represents the Left mouse button.
    LeftButton,
    /// Represents the Right mouse button.
    RightButton,
    /// Represents the Middle mouse button.
    MiddleButton,
    /// Represents mouse extension button 1.
    X1Button,
    /// Represents mouse extension button 2.
    X2Button,

    /// Represents the Left arrow key.
    Left,
    /// Represents the Right arrow key.
    Right,
    /// Represents the Up arrow key.
    Up,
    /// Represents the Down arrow key.
    Down,

    /// Represents other keys not specified by the above variants.
    /// - `i32`: The raw key code.
    Unknown(i32),
}

/// Represents the different mouse buttons that can be used in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Represents the left mouse button.
    Left,
    /// Represents the right mouse button.
    Right,
    /// Represents the middle mouse button.
    Middle,
    /// Represents other mouse buttons.
    /// - `u16`: The raw button code.
    Other(u16),
}

/// Represents the different directions the mouse wheel can be scrolled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseWheel {
    /// Represents scrolling up.
    Up,
    /// Represents scrolling down.
    Down,
}

/// Represents the different modifier key states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierKey {
    /// Represents the Shift key being pressed.
    Shift,
    /// Represents the Ctrl key being pressed.
    Ctrl,
    /// Represents the Alt key being pressed.
    Alt,
    /// Represents the Windows key being pressed.
    Win,
    /// Represents a mouse button being pressed.
    /// - `MouseButton`: The specific mouse button.
    Mouse(MouseButton),
    /// Represents no modifier keys being pressed.
    None,
}

/// Represents the different types of window resize operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSize {
    /// Represents a general resize operation.
    Resize,
    /// Represents the window being minimized.
    Minimize,
    /// Represents the window being maximized.
    Maximize,
    /// Represents the window being restored to its normal state.
    Restore,
    /// Represents the window being hidden in maximized mode.
    MaxHide,
    /// Represents the window being shown in maximized mode.
    MaxShow,
}
