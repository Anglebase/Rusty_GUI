use std::{any::type_name, ptr::null_mut, sync::Mutex};

use winapi::{
    shared::{
        minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::LPCWSTR,
        windef::{HBRUSH, HWND, RECT},
    },
    um::{libloaderapi::GetModuleHandleW, winuser::*},
};

use crate::{
    debug,
    events::{ArrowType, Button, Key, SizeType, Wheel},
    App,
};
use crate::{Graph, Point, Rect, Size, Window};

/// Trait `WinProc` defines the behavior for windows.
/// You can implement this trait for your own window types.
/// You must implement `Clone` trait for your window types before implementing this trait,
/// because some types need `Clone` trait to safely copy (such as `String`).
/// All methods in this trait have default empty implementations,
/// so you don't have to implement all of them.
/// # Note
/// The first parameter `self` of these methods is the reference to theirselves object instance.
/// And the second parameter `this` is the reference to the `Window` object which binds with the object instance.
/// # Example
/// ```
/// use rusty_gui::*;
///
/// #[derive(Clone)]
/// struct MyWindow;
///
/// impl WinProc for MyWindow {
///     fn draw(&mut self, _: &mut rusty_gui::Window, g: &mut rusty_gui::Graph) {
///         g.text("Hello, Rusty GUI!", p!(50, 50));
///     }
/// }
///
/// fn main() {
///     let window = MyWindow.create_window("My Window", rect!(200, 200, 800, 600), None);
///     window.show();
///     App::run();
/// }
/// ```
#[allow(unused)]
pub trait WinProc: Clone + 'static {
    /// This method is called when the window is created.
    fn create(&mut self, this: &mut Window) {}
    /// This method is called when the window is destroyed.
    fn destroy(&mut self, this: &mut Window) {}

    /// This method is called when the window needs to be redrawn.
    /// `g` is the graphics device context for the window.
    fn draw(&mut self, this: &mut Window, g: &mut Graph) {}
    /// This method is called when any timer in the window is triggered.
    /// `timer_id` is the ID of the timer that triggered.
    fn timer(&mut self, this: &mut Window, timer_id: usize) {}

    /// This method is called when a mouse button is pressed.
    fn button_down(&mut self, this: &mut Window, button: Button) {}
    /// This method is called when a mouse button is released.
    fn button_up(&mut self, this: &mut Window, button: Button) {}
    /// This method is called when a mouse button is double-clicked.
    fn button_dbclk(&mut self, this: &mut Window, button: Button) {}
    /// This method is called when the mouse is moved.
    fn mouse_move(&mut self, this: &mut Window, point: Point, key: Option<Key>) {}
    /// This method is called when the mouse wheel is scrolled.
    fn mouse_wheel(&mut self, this: &mut Window, point: Point, wheel: Wheel, key: Option<Key>) {}

    /// This method is called when a key is pressed.
    fn key_down(&mut self, this: &mut Window, key: Key) {}
    /// This method is called when a key is released.
    fn key_up(&mut self, this: &mut Window, key: Key) {}
    /// This method is called when a character is input.
    fn input(&mut self, this: &mut Window, ch: char) {}

    /// This method is called when the window is resized.
    fn window_resize(&mut self, this: &mut Window, size: Size, size_type: SizeType) {}
    /// This method is called when the window is moved.
    fn window_move(&mut self, this: &mut Window, point: Point) {}
}

static mut WIN_COUNT: Mutex<u32> = Mutex::new(0);

/// Trait `WinImplPrivate` defines private interfaces related to window interaction.
/// All types that implement `WinProc` will automatically implement this trait.
#[allow(unused)]
pub trait WinImplPrivate: WinProc {
    /// The window procedure function.
    /// This function should not be called directly.
    unsafe extern "system" fn winproc(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // Get the pointer to the `Self` object
        let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
        let this = this as *mut Box<Self>;
        if this.is_null() {
            debug!("Window procedure called with unloaded pointer.");
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        let it = this.as_mut().unwrap();
        let mut w = Window { hwnd };
        // Handle the message
        match msg {
            WM_DESTROY => {
                it.destroy(&mut w);
                // Decrement the window count.
                let count = {
                    let mut count = WIN_COUNT.lock().unwrap();
                    *count -= 1;
                    *count
                };
                // Quit the application when the last window is closed.
                if count == 0 {
                    App::quit();
                }
                Box::from_raw(this);
                return 0;
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT {
                    hdc: null_mut(),
                    fErase: 0,
                    rcPaint: RECT {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                    },
                    fRestore: 0,
                    fIncUpdate: 0,
                    rgbReserved: [0; 32],
                };
                let hdc = BeginPaint(hwnd, &mut ps);
                let mut g = Graph { hdc };
                it.draw(&mut w, &mut g);
                EndPaint(hwnd, &ps);
                return 0;
            }
            WM_TIMER => {
                let timer_id = wparam as usize;
                let mut w = Window { hwnd };
                it.timer(&mut w, timer_id);
            }
            WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDBLCLK
            | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP => {
                return Self::handle_mouse_event(it, hwnd, msg, wparam, lparam);
            }
            WM_KEYDOWN | WM_KEYUP => return Self::handle_key_event(it, hwnd, msg, wparam, lparam),
            WM_CHAR => {
                let input = std::char::from_u32(wparam as u32).unwrap();
                it.input(&mut w, input);
            }
            WM_MOUSEMOVE => {
                let pos = Point {
                    x: LOWORD(lparam as u32).into(),
                    y: HIWORD(lparam as u32).into(),
                };
                let ext = match wparam {
                    MK_CONTROL => Some(Key::Ctrl),
                    MK_LBUTTON => Some(Key::LeftButton),
                    MK_MBUTTON => Some(Key::MiddleButton),
                    MK_RBUTTON => Some(Key::RightButton),
                    MK_SHIFT => Some(Key::Shift),
                    MK_XBUTTON1 => Some(Key::X1Button),
                    MK_XBUTTON2 => Some(Key::X2Button),
                    _ => None,
                };
                it.mouse_move(&mut w, pos, ext);
            }
            WM_MOUSEWHEEL => {
                let pos = Point {
                    x: LOWORD(lparam as u32).into(),
                    y: HIWORD(lparam as u32).into(),
                };
                let ext = match LOWORD(wparam as u32) as usize {
                    MK_CONTROL => Some(Key::Ctrl),
                    MK_LBUTTON => Some(Key::LeftButton),
                    MK_MBUTTON => Some(Key::MiddleButton),
                    MK_RBUTTON => Some(Key::RightButton),
                    MK_SHIFT => Some(Key::Shift),
                    MK_XBUTTON1 => Some(Key::X1Button),
                    MK_XBUTTON2 => Some(Key::X2Button),
                    _ => None,
                };
                let delta = HIWORD(wparam as u32) as i16;
                it.mouse_wheel(
                    &mut w,
                    pos,
                    if delta > 0 {
                        Wheel::Up(delta)
                    } else {
                        Wheel::Down(delta)
                    },
                    ext,
                );
            }
            WM_SIZE => {
                let size = Size {
                    width: LOWORD(lparam as u32).into(),
                    height: HIWORD(lparam as u32).into(),
                };
                let st = match wparam {
                    SIZE_MAXIMIZED => SizeType::Maximized,
                    SIZE_MINIMIZED => SizeType::Minimized,
                    SIZE_RESTORED => SizeType::Restored,
                    SIZE_MAXHIDE => SizeType::MaxHide,
                    SIZE_MAXSHOW => SizeType::MaxShow,
                    _ => SizeType::Unknown,
                };
                it.window_resize(&mut w, size, st);
            }
            WM_MOVE => {
                let pos = Point {
                    x: LOWORD(lparam as u32).into(),
                    y: HIWORD(lparam as u32).into(),
                };
                it.window_move(&mut w, pos);
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    // this function is used to handle mouse events
    unsafe fn handle_mouse_event(
        this: &mut Box<Self>,
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let point = Point {
            x: LOWORD(lparam as u32).into(),
            y: HIWORD(lparam as u32).into(),
        };
        let mut it = Window { hwnd };
        match msg {
            WM_LBUTTONDOWN => {
                this.button_down(&mut it, Button::Left(point));
                return 0;
            }
            WM_LBUTTONUP => {
                this.button_up(&mut it, Button::Left(point));
                return 0;
            }
            WM_LBUTTONDBLCLK => {
                this.button_dbclk(&mut it, Button::Left(point));
                return 0;
            }
            WM_MBUTTONDOWN => {
                this.button_down(&mut it, Button::Middle(point));
                return 0;
            }
            WM_MBUTTONUP => {
                this.button_up(&mut it, Button::Middle(point));
                return 0;
            }
            WM_MBUTTONDBLCLK => {
                this.button_dbclk(&mut it, Button::Middle(point));
                return 0;
            }
            WM_RBUTTONDOWN => {
                this.button_down(&mut it, Button::Right(point));
                return 0;
            }
            WM_RBUTTONUP => {
                this.button_up(&mut it, Button::Right(point));
                return 0;
            }
            WM_RBUTTONDBLCLK => {
                this.button_dbclk(&mut it, Button::Right(point));
                return 0;
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    // this function is used to convert a virtual key code to a `Key` value
    fn vk_to_key(vk: i32) -> Key {
        match vk {
            0x41..=0x5A => Key::Alpha(vk as u8 as char),
            0x30..=0x39 => Key::N(vk as u8 as char),
            VK_F1 => Key::F(1),
            VK_F2 => Key::F(2),
            VK_F3 => Key::F(3),
            VK_F4 => Key::F(4),
            VK_F5 => Key::F(5),
            VK_F6 => Key::F(6),
            VK_F7 => Key::F(7),
            VK_F8 => Key::F(8),
            VK_F9 => Key::F(9),
            VK_F10 => Key::F(10),
            VK_F11 => Key::F(11),
            VK_F12 => Key::F(12),
            VK_NUMPAD0 => Key::Num(0),
            VK_NUMPAD1 => Key::Num(1),
            VK_NUMPAD2 => Key::Num(2),
            VK_NUMPAD3 => Key::Num(3),
            VK_NUMPAD4 => Key::Num(4),
            VK_NUMPAD5 => Key::Num(5),
            VK_NUMPAD6 => Key::Num(6),
            VK_NUMPAD7 => Key::Num(7),
            VK_NUMPAD8 => Key::Num(8),
            VK_NUMPAD9 => Key::Num(9),

            VK_SHIFT => Key::Shift,
            VK_CONTROL => Key::Ctrl,
            VK_MENU => Key::Alt,

            VK_OEM_1 => Key::Symbol(';'),
            VK_OEM_2 => Key::Symbol('/'),
            VK_OEM_3 => Key::Symbol('`'),
            VK_OEM_4 => Key::Symbol('['),
            VK_OEM_5 => Key::Symbol('\\'),
            VK_OEM_6 => Key::Symbol(']'),
            VK_OEM_7 => Key::Symbol('\''),
            VK_OEM_PLUS => Key::Symbol('+'),
            VK_OEM_COMMA => Key::Symbol(','),
            VK_OEM_MINUS => Key::Symbol('-'),
            VK_OEM_PERIOD => Key::Symbol('.'),

            VK_ADD => Key::NumAdd,
            VK_SUBTRACT => Key::NumSub,
            VK_MULTIPLY => Key::NumMul,
            VK_DIVIDE => Key::NumDiv,
            VK_DECIMAL => Key::NumDot,

            VK_BACK => Key::Backspace,
            VK_TAB => Key::Tab,
            VK_RETURN => Key::Enter,
            VK_SPACE => Key::Space,

            VK_ESCAPE => Key::Esc,
            VK_CAPITAL => Key::CapsLock,
            VK_LCONTROL => Key::LeftCtrl,
            VK_LSHIFT => Key::LeftShift,
            VK_LMENU => Key::LeftAlt,
            VK_RCONTROL => Key::RightCtrl,
            VK_RSHIFT => Key::RightShift,
            VK_RMENU => Key::RightAlt,
            VK_SCROLL => Key::ScrollLock,
            VK_NUMLOCK => Key::NumLock,
            VK_DELETE => Key::Delete,
            VK_INSERT => Key::Insert,
            VK_HOME => Key::Home,
            VK_END => Key::End,
            VK_PRIOR => Key::PageUp,
            VK_NEXT => Key::PageDown,

            VK_LBUTTON => Key::LeftButton,
            VK_RBUTTON => Key::RightButton,
            VK_MBUTTON => Key::MiddleButton,
            VK_XBUTTON1 => Key::X1Button,
            VK_XBUTTON2 => Key::X2Button,

            VK_LEFT => Key::Arrow(ArrowType::Left),
            VK_UP => Key::Arrow(ArrowType::Up),
            VK_RIGHT => Key::Arrow(ArrowType::Right),
            VK_DOWN => Key::Arrow(ArrowType::Down),

            _ => Key::Unknown(vk),
        }
    }

    // this function is used to handle key events
    unsafe fn handle_key_event(
        this: &mut Box<Self>,
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let key = Self::vk_to_key(wparam as i32);
        let mut it = Window { hwnd };
        match msg {
            WM_KEYDOWN => {
                this.key_down(&mut it, key);
                return 0;
            }
            WM_KEYUP => {
                this.key_up(&mut it, key);
                return 0;
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    /// Register the window class for the current window type.
    /// If the class has already been registered or registration fails, this function returns `false`.
    /// Otherwise, it returns `true`.
    /// Note that this function should not be called directly. It is called by the `create_window` method.
    fn register_this(class_name: LPCWSTR) -> bool {
        let hinstance = unsafe { GetModuleHandleW(null_mut()) };
        // Check if the class has already been registered.
        let mut wndclass = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(Self::winproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name,
            hIconSm: null_mut(),
        };
        if unsafe { GetClassInfoExW(hinstance, class_name, &mut wndclass as *mut _) != 0 } {
            return false;
        }
        debug!("Registering window class: {}", type_name::<Self>());
        // Register the class.
        wndclass.style = CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS;
        wndclass.lpfnWndProc = Some(Self::winproc);
        wndclass.hInstance = hinstance;
        wndclass.hIcon = unsafe { LoadIconW(null_mut(), IDI_APPLICATION) };
        wndclass.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
        wndclass.hbrBackground = (COLOR_WINDOW + 1) as HBRUSH;
        wndclass.lpszClassName = class_name;
        let atom = unsafe { RegisterClassExW(&wndclass) };
        if atom == 0 {
            return false;
        }
        true
    }
}
/// Trait `WinImpl` defines interfaces related to window interaction.
/// All types that implement `WinProc` will automatically implement this trait.
/// All methods in this trait needn't be implemented by users.
#[allow(unused)]
pub trait WinImpl: WinImplPrivate {
    /// Create a window with the given title, position, size, and parent window.
    /// If parent is `None`, the window is a top-level window.
    /// Otherwise, the window is a child window.
    fn create_window(&mut self, title: &str, rect: Rect, parent: Option<&Window>) -> Window {
        let title: LPCWSTR = title
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr() as _;
        // Get the class name.
        let class_name: String = type_name::<Self>().into();
        let class_name: LPCWSTR = class_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr() as _;
        // Register the class.
        Self::register_this(class_name);
        // Create the window.
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name,
                title,
                match parent {
                    Some(_) => WS_CHILD,
                    None => WS_OVERLAPPEDWINDOW,
                },
                rect.pos.x,
                rect.pos.y,
                rect.size.width,
                rect.size.height,
                // If parent is None, the window is a top-level window.
                // Otherwise, the window is a child window.
                match parent {
                    Some(parent) => parent.hwnd,
                    None => null_mut(),
                },
                null_mut(),
                GetModuleHandleW(null_mut()),
                null_mut(),
            )
        };
        // Increment the window count.
        unsafe {
            let mut count = WIN_COUNT.lock().unwrap();
            *count += 1;
        }
        let mut w = Window { hwnd };
        self.create(&mut w);
        // Set the user data to the pointer to the `Self` object.
        debug!(
            "Loading object instance pointer for window {:?}.",
            &self as *const _
        );
        unsafe {
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                Box::into_raw(Box::new(Box::new(self.clone()))) as _,
            )
        };
        Window { hwnd }
    }
}

// Automatically implement `WinImplPrivate` for all types that implement `WinProc`.
impl<T: WinProc> WinImplPrivate for T {}
// Automatically implement `WinImpl` for all types that implement `WinImplPrivate`.
impl<T: WinImplPrivate> WinImpl for T {}
