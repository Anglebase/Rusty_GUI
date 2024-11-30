use crate::{winproc::WinProc, ArrowType, Button, Key, SizeType, Wheel};
/// This file contains the core implementation of this crate.
/// author: Anglebase (https://github.com/Anglebase)
/// ---------------------------------------------------------
use std::{collections::HashMap, sync::Mutex};
use winapi::{
    shared::{minwindef::*, windef::*},
    um::{libloaderapi::*, wingdi::*, winnt::*, winuser::*},
};

pub(crate) fn string_to_wchar(s: &str) -> Vec<WCHAR> {
    let mut result = Vec::new();
    for c in s.chars() {
        result.push(c as WCHAR);
    }
    result.push(0);
    result
}

// Trait
pub(crate) unsafe fn register_window_class(name: &str, winproc: WNDPROC) -> ATOM {
    let class_name = string_to_wchar(name);
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
        lpfnWndProc: winproc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: GetModuleHandleW(std::ptr::null()),
        hIcon: std::ptr::null_mut(),
        hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
        hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
        lpszMenuName: std::ptr::null_mut(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: std::ptr::null_mut(),
    };
    RegisterClassExW(&class)
}

// Window new
pub(crate) unsafe fn create_window(
    class_name: &str,
    title: &str,
    width: i32,
    height: i32,
    parent: HWND,
) -> HWND {
    let window_name = string_to_wchar(title);
    let hwnd = CreateWindowExW(
        0,
        string_to_wchar(class_name).as_ptr(),
        window_name.as_ptr(),
        if !parent.is_null() {
            WS_CHILD
        } else {
            WS_OVERLAPPEDWINDOW
        },
        100,
        100,
        width,
        height,
        parent,
        std::ptr::null_mut(),
        GetModuleHandleW(std::ptr::null()),
        std::ptr::null_mut(),
    );
    hwnd
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[macro_export]
macro_rules! p {
    ($x:expr, $y:expr $(,)?) => {
        Point { x: $x, y: $y }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[macro_export]
macro_rules! s {
    ($w:expr, $h:expr $(,)?) => {
        Size {
            width: $w,
            height: $h,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

#[allow(unused)]
pub struct Graphics {
    pub(crate) hdc: HDC,
    pub(crate) hwnd: HWND,
}

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct Window {
    pub(crate) hwnd: HWND,
}

pub(crate) const CLASS_NAME: &str = "rusty_gui_window_class";
pub(crate) static mut G_MAP: Mutex<Option<HashMap<HWND, Box<dyn WinProc>>>> = Mutex::new(None);

pub(crate) fn gmap_init() {
    unsafe {
        *G_MAP.lock().unwrap() = Some(HashMap::new());
    }
}
pub(crate) fn gmap_insert(hwnd: HWND, proc: Box<dyn WinProc>) {
    unsafe {
        let mut map = G_MAP.lock().unwrap();
        map.as_mut().unwrap().insert(hwnd, proc);
    }
}
pub(crate) fn gmap_remove(hwnd: HWND) {
    unsafe {
        let mut map = G_MAP.lock().unwrap();
        map.as_mut().unwrap().remove(&hwnd);
    }
}
pub(crate) fn gmap_is_null() -> bool {
    unsafe { G_MAP.lock().unwrap().is_none() }
}

pub(crate) static mut G_MAINWINDOW: Mutex<Option<Window>> = Mutex::new(None);
pub(crate) unsafe extern "system" fn gwndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if !G_MAP.lock().unwrap().as_ref().unwrap().contains_key(&hwnd) {
        println!("Warning: HWND Not Found in G_MAP.");
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    match msg {
        WM_PAINT => {
            // double buffering
            let mut ps = PAINTSTRUCT {
                hdc: std::ptr::null_mut(),
                fErase: 0,
                rcPaint: std::mem::zeroed(),
                fRestore: 0,
                fIncUpdate: 0,
                rgbReserved: [0; 32],
            };
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetClientRect(hwnd, &mut rect);
            let hdc = BeginPaint(hwnd, &mut ps);
            let buffer = CreateCompatibleDC(hdc);
            let bitmap =
                CreateCompatibleBitmap(hdc, rect.right - rect.left, rect.bottom - rect.top);
            let prebmp = SelectObject(buffer, bitmap as *mut _) as HBITMAP;

            let mut graphics = Graphics { hdc: buffer, hwnd };
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .draw(&mut graphics);
            }
            BitBlt(
                hdc,
                0,
                0,
                rect.right - rect.left,
                rect.bottom - rect.top,
                buffer,
                0,
                0,
                SRCCOPY,
            );
            SelectObject(buffer, prebmp as *mut _);
            DeleteObject(bitmap as *mut _);
            DeleteDC(buffer);
            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            gmap_remove(hwnd);
            if G_MAINWINDOW.lock().unwrap().as_ref().unwrap().hwnd == hwnd {
                PostQuitMessage(0);
            }
            0
        }
        WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDBLCLK | WM_MBUTTONDOWN
        | WM_MBUTTONUP | WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP => {
            handle_mouse_event(hwnd, msg, wparam, lparam)
        }
        WM_KEYDOWN | WM_KEYUP => handle_key_event(hwnd, msg, wparam, lparam),
        WM_CHAR => {
            let input: &str = &std::char::from_u32(wparam as u32).unwrap().to_string();
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .input(input);
            }
            0
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
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .mouse_move(pos, ext);
            }
            0
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
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .mouse_wheel(
                        pos,
                        if delta > 0 {
                            Wheel::Up(delta)
                        } else {
                            Wheel::Down(delta)
                        },
                        ext,
                    );
            }
            0
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
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .window_resize(size, st);
            }
            0
        }
        WM_MOVE => {
            let pos = Point {
                x: LOWORD(lparam as u32).into(),
                y: HIWORD(lparam as u32).into(),
            };
            {
                G_MAP
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .get_mut(&hwnd)
                    .as_mut()
                    .unwrap()
                    .window_move(pos);
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// this function is used to handle mouse events
unsafe fn handle_mouse_event(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let point = Point {
        x: LOWORD(lparam as u32).into(),
        y: HIWORD(lparam as u32).into(),
    };
    match msg {
        WM_LBUTTONDOWN => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_down(Button::Left(point));
            return 0;
        }
        WM_LBUTTONUP => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_up(Button::Left(point));
            return 0;
        }
        WM_LBUTTONDBLCLK => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_dbclk(Button::Left(point));
            return 0;
        }
        WM_MBUTTONDOWN => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_down(Button::Middle(point));
            return 0;
        }
        WM_MBUTTONUP => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_up(Button::Middle(point));
            return 0;
        }
        WM_MBUTTONDBLCLK => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_dbclk(Button::Middle(point));
            return 0;
        }
        WM_RBUTTONDOWN => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_down(Button::Right(point));
            return 0;
        }
        WM_RBUTTONUP => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_up(Button::Right(point));
            return 0;
        }
        WM_RBUTTONDBLCLK => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .button_dbclk(Button::Right(point));
            return 0;
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

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
unsafe fn handle_key_event(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let key = vk_to_key(wparam as i32);
    match msg {
        WM_KEYDOWN => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .key_down(key);
            return 0;
        }
        WM_KEYUP => {
            G_MAP
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_mut(&hwnd)
                .as_mut()
                .unwrap()
                .key_up(key);
            return 0;
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}
