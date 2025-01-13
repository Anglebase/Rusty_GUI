use crate::{rect, AbstractElement, KeyCode, Point, Rect, Size, WindowID};
use std::any::Any;
use std::{
    os::raw::c_void,
    ptr::{null, null_mut},
};
use winapi::{
    shared::windef::RECT,
    um::{wincon::FreeConsole, winuser::*},
};

use super::*;

pub fn get_rect(hwnd: *mut c_void) -> Rect {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetClientRect(hwnd as _, &mut rect as _);
    }
    rect!(
        rect.left,
        rect.top,
        rect.right - rect.left,
        rect.bottom - rect.top
    )
}

pub fn get_absolute_rect(hwnd: *mut c_void) -> Rect {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetWindowRect(hwnd as _, &mut rect as _);
    }
    rect!(
        rect.left,
        rect.top,
        rect.right - rect.left,
        rect.bottom - rect.top
    )
}

pub fn show_and_update(hwnd: *mut c_void) {
    unsafe {
        ShowWindow(hwnd as _, SW_SHOW);
        UpdateWindow(hwnd as _);
    }
}

pub fn notifier_exit(code: i32) {
    unsafe {
        PostQuitMessage(code);
    }
}

pub fn close_cmd() {
    unsafe {
        FreeConsole();
    }
}

pub fn set_no_auto_dpi_scale() {
    unsafe {
        SetProcessDPIAware();
    }
}

pub fn get_dpi_scale() -> f32 {
    unsafe { 96.0 / GetDpiForSystem() as f32 }
}

pub fn set_window_rect(hwnd: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    unsafe {
        SetWindowPos(
            hwnd as _,
            HWND_TOP,
            x,
            y,
            w,
            h,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn set_window_title(hwnd: *mut c_void, title: &str) {
    let title = title
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    unsafe {
        SetWindowTextW(hwnd as _, title.as_ptr());
    }
}

pub fn get_window_title(hwnd: *mut c_void) -> String {
    let len = unsafe { GetWindowTextLengthW(hwnd as _) };
    let mut title = vec![0u16; len as usize + 1];
    unsafe {
        GetWindowTextW(hwnd as _, title.as_mut_ptr(), len + 1);
    }
    String::from_utf16_lossy(&title)
}

pub fn update_window(hwnd: *mut c_void) {
    unsafe {
        RedrawWindow(
            hwnd as _,
            null(),
            null_mut(),
            RDW_INVALIDATE | RDW_UPDATENOW,
        );
    }
}

pub fn set_window_visible(hwnd: *mut c_void, visible: bool) {
    unsafe {
        ShowWindow(hwnd as _, if visible { SW_SHOW } else { SW_HIDE });
    }
}

pub fn set_window_minimized(hwnd: *mut c_void) {
    unsafe {
        ShowWindow(hwnd as _, SW_MINIMIZE);
    }
}

pub fn set_window_maximized(hwnd: *mut c_void) {
    unsafe {
        ShowWindow(hwnd as _, SW_MAXIMIZE);
    }
}

pub fn set_window_restored(hwnd: *mut c_void) {
    unsafe {
        ShowWindow(hwnd as _, SW_RESTORE);
    }
}

pub fn disable_window(hwnd: *mut c_void) {
    unsafe {
        EnableWindow(hwnd as _, 0);
    }
}

pub fn enable_window(hwnd: *mut c_void) {
    unsafe {
        EnableWindow(hwnd as _, 1);
    }
}

pub fn vk_to_key(vk: i32) -> KeyCode {
    match vk {
        0x41..=0x5A => KeyCode::Alpha(vk as u8 as char),
        0x30..=0x39 => KeyCode::N(vk as u8 as char),
        VK_F1 => KeyCode::F(1),
        VK_F2 => KeyCode::F(2),
        VK_F3 => KeyCode::F(3),
        VK_F4 => KeyCode::F(4),
        VK_F5 => KeyCode::F(5),
        VK_F6 => KeyCode::F(6),
        VK_F7 => KeyCode::F(7),
        VK_F8 => KeyCode::F(8),
        VK_F9 => KeyCode::F(9),
        VK_F10 => KeyCode::F(10),
        VK_F11 => KeyCode::F(11),
        VK_F12 => KeyCode::F(12),
        VK_NUMPAD0 => KeyCode::Num(0),
        VK_NUMPAD1 => KeyCode::Num(1),
        VK_NUMPAD2 => KeyCode::Num(2),
        VK_NUMPAD3 => KeyCode::Num(3),
        VK_NUMPAD4 => KeyCode::Num(4),
        VK_NUMPAD5 => KeyCode::Num(5),
        VK_NUMPAD6 => KeyCode::Num(6),
        VK_NUMPAD7 => KeyCode::Num(7),
        VK_NUMPAD8 => KeyCode::Num(8),
        VK_NUMPAD9 => KeyCode::Num(9),

        VK_SHIFT => KeyCode::Shift,
        VK_CONTROL => KeyCode::Ctrl,
        VK_MENU => KeyCode::Alt,

        VK_OEM_1 => KeyCode::Symbol(';'),
        VK_OEM_2 => KeyCode::Symbol('/'),
        VK_OEM_3 => KeyCode::Symbol('`'),
        VK_OEM_4 => KeyCode::Symbol('['),
        VK_OEM_5 => KeyCode::Symbol('\\'),
        VK_OEM_6 => KeyCode::Symbol(']'),
        VK_OEM_7 => KeyCode::Symbol('\''),
        VK_OEM_PLUS => KeyCode::Symbol('+'),
        VK_OEM_COMMA => KeyCode::Symbol(','),
        VK_OEM_MINUS => KeyCode::Symbol('-'),
        VK_OEM_PERIOD => KeyCode::Symbol('.'),

        VK_ADD => KeyCode::NumAdd,
        VK_SUBTRACT => KeyCode::NumSub,
        VK_MULTIPLY => KeyCode::NumMul,
        VK_DIVIDE => KeyCode::NumDiv,
        VK_DECIMAL => KeyCode::NumDot,

        VK_BACK => KeyCode::Backspace,
        VK_TAB => KeyCode::Tab,
        VK_RETURN => KeyCode::Enter,
        VK_SPACE => KeyCode::Space,

        VK_ESCAPE => KeyCode::Esc,
        VK_CAPITAL => KeyCode::CapsLock,
        VK_LCONTROL => KeyCode::LeftCtrl,
        VK_LSHIFT => KeyCode::LeftShift,
        VK_LMENU => KeyCode::LeftAlt,
        VK_RCONTROL => KeyCode::RightCtrl,
        VK_RSHIFT => KeyCode::RightShift,
        VK_RMENU => KeyCode::RightAlt,
        VK_SCROLL => KeyCode::ScrollLock,
        VK_NUMLOCK => KeyCode::NumLock,
        VK_DELETE => KeyCode::Delete,
        VK_INSERT => KeyCode::Insert,
        VK_HOME => KeyCode::Home,
        VK_END => KeyCode::End,
        VK_PRIOR => KeyCode::PageUp,
        VK_NEXT => KeyCode::PageDown,
        VK_CLEAR => KeyCode::Clear,

        VK_LBUTTON => KeyCode::LeftButton,
        VK_RBUTTON => KeyCode::RightButton,
        VK_MBUTTON => KeyCode::MiddleButton,
        VK_XBUTTON1 => KeyCode::X1Button,
        VK_XBUTTON2 => KeyCode::X2Button,

        VK_LEFT => KeyCode::Left,
        VK_UP => KeyCode::Up,
        VK_RIGHT => KeyCode::Right,
        VK_DOWN => KeyCode::Down,

        _ => KeyCode::Unknown(vk),
    }
}

pub fn key_to_vk(key: KeyCode) -> i32 {
    match key {
        KeyCode::Alpha(c) => c as i32,
        KeyCode::N(c) => c as i32,
        KeyCode::F(n) => match n {
            1 => VK_F1,
            2 => VK_F2,
            3 => VK_F3,
            4 => VK_F4,
            5 => VK_F5,
            6 => VK_F6,
            7 => VK_F7,
            8 => VK_F8,
            9 => VK_F9,
            10 => VK_F10,
            11 => VK_F11,
            12 => VK_F12,
            _ => 0,
        },
        KeyCode::Num(n) => match n {
            0 => VK_NUMPAD0,
            1 => VK_NUMPAD1,
            2 => VK_NUMPAD2,
            3 => VK_NUMPAD3,
            4 => VK_NUMPAD4,
            5 => VK_NUMPAD5,
            6 => VK_NUMPAD6,
            7 => VK_NUMPAD7,
            8 => VK_NUMPAD8,
            9 => VK_NUMPAD9,
            _ => 0,
        },
        KeyCode::Shift => VK_SHIFT,
        KeyCode::Ctrl => VK_CONTROL,
        KeyCode::Alt => VK_MENU,
        KeyCode::Symbol(c) => match c {
            ';' => VK_OEM_1,
            '/' => VK_OEM_2,
            '`' => VK_OEM_3,
            '[' => VK_OEM_4,
            '\\' => VK_OEM_5,
            ']' => VK_OEM_6,
            '\'' => VK_OEM_7,
            '+' => VK_OEM_PLUS,
            ',' => VK_OEM_COMMA,
            '-' => VK_OEM_MINUS,
            '.' => VK_OEM_PERIOD,
            _ => 0,
        },
        KeyCode::NumAdd => VK_ADD,
        KeyCode::NumSub => VK_SUBTRACT,
        KeyCode::NumMul => VK_MULTIPLY,
        KeyCode::NumDiv => VK_DIVIDE,
        KeyCode::NumDot => VK_DECIMAL,
        KeyCode::Backspace => VK_BACK,
        KeyCode::Tab => VK_TAB,
        KeyCode::Enter => VK_RETURN,
        KeyCode::Space => VK_SPACE,
        KeyCode::Esc => VK_ESCAPE,
        KeyCode::CapsLock => VK_CAPITAL,
        KeyCode::LeftCtrl => VK_LCONTROL,
        KeyCode::LeftShift => VK_LSHIFT,
        KeyCode::LeftAlt => VK_LMENU,
        KeyCode::RightCtrl => VK_RCONTROL,
        KeyCode::RightShift => VK_RSHIFT,
        KeyCode::RightAlt => VK_RMENU,
        KeyCode::ScrollLock => VK_SCROLL,
        KeyCode::NumLock => VK_NUMLOCK,
        KeyCode::Delete => VK_DELETE,
        KeyCode::Insert => VK_INSERT,
        KeyCode::Home => VK_HOME,
        KeyCode::End => VK_END,
        KeyCode::PageUp => VK_PRIOR,
        KeyCode::PageDown => VK_NEXT,
        KeyCode::Clear => VK_CLEAR,
        KeyCode::LeftButton => VK_LBUTTON,
        KeyCode::RightButton => VK_RBUTTON,
        KeyCode::MiddleButton => VK_MBUTTON,
        KeyCode::X1Button => VK_XBUTTON1,
        KeyCode::X2Button => VK_XBUTTON2,
        KeyCode::Left => VK_LEFT,
        KeyCode::Up => VK_UP,
        KeyCode::Right => VK_RIGHT,
        KeyCode::Down => VK_DOWN,
        _ => 0,
    }
}

pub fn register_hotkey_for_window(
    hwnd: *mut c_void,
    id: i32,
    key: KeyCode,
    modifiers: HotKeyFlags,
) {
    let vk = key_to_vk(key);
    if vk == 0 {
        return;
    }
    let modifiers = if modifiers.alt { MOD_ALT } else { 0 }
        | if modifiers.ctrl { MOD_CONTROL } else { 0 }
        | if modifiers.shift { MOD_SHIFT } else { 0 }
        | if modifiers.win { MOD_WIN } else { 0 }
        | MOD_NOREPEAT;
    unsafe {
        RegisterHotKey(hwnd as _, id, modifiers as u32, vk as _);
    }
}

pub fn set_window_timer(hwnd: *mut c_void, id: usize, interval: u32) {
    unsafe {
        SetTimer(hwnd as _, id, interval, None);
    }
}

pub fn kill_window_timer(hwnd: *mut c_void, id: usize) {
    unsafe {
        KillTimer(hwnd as _, id);
    }
}

#[allow(unused)]
pub fn set_window_style(hwnd: *mut c_void, style: WindowStyle) {
    let mut style_ = 0;
    if style.border {
        style_ |= WS_BORDER;
    }
    if style.caption {
        style_ |= WS_CAPTION;
    }
    if style.child {
        style_ |= WS_CHILD;
    }
    if style.resize {
        style_ |= WS_THICKFRAME;
    }
    if style.sysmenu {
        style_ |= WS_SYSMENU;
    }
    if style.minbox {
        style_ |= WS_MINIMIZEBOX;
    }
    if style.maxbox {
        style_ |= WS_MAXIMIZEBOX;
    }
    let ex_style = 0;
    if style.topmost {
        unsafe {
            SetWindowPos(hwnd as _, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
        }
    }
    unsafe {
        SetWindowLongW(hwnd as _, GWL_STYLE, style_ as _);
        SetWindowLongW(hwnd as _, GWL_EXSTYLE, ex_style as _);
    }
}

#[allow(unused)]
pub fn get_window_style(hwnd: *mut c_void) -> WindowStyle {
    let style = unsafe { GetWindowLongW(hwnd as _, GWL_STYLE) as u32 };
    let ex_style = unsafe { GetWindowLongW(hwnd as _, GWL_EXSTYLE) as u32 };
    let mut style_ = WindowStyle::default();
    if style & WS_BORDER != 0 {
        style_.border = true;
    }
    if style & WS_CAPTION != 0 {
        style_.caption = true;
    }
    if style & WS_CHILD != 0 {
        style_.child = true;
    }
    if style & WS_THICKFRAME != 0 {
        style_.resize = true;
    }
    if style & WS_SYSMENU != 0 {
        style_.sysmenu = true;
    }
    if style & WS_MINIMIZEBOX != 0 {
        style_.minbox = true;
    }
    if style & WS_MAXIMIZEBOX != 0 {
        style_.maxbox = true;
    }
    if ex_style & WS_EX_TOPMOST != 0 {
        style_.topmost = true;
    }
    style_
}

pub fn set_window_focus(hwnd: *mut c_void) {
    unsafe {
        SetFocus(hwnd as _);
    }
}

pub fn is_window_onfocus(hwnd: *mut c_void) -> bool {
    let h = unsafe { GetFocus() };
    h == hwnd as _
}

pub fn for_each_child_window(hwnd: *mut c_void, mut f: Box<dyn FnMut(&mut dyn AbstractElement)>) {
    unsafe {
        EnumChildWindows(
            hwnd as _,
            Some(enum_windows_callback),
            &mut f as *mut _ as _,
        );
    }
}

pub fn send_user_def_msg(hwnd: *mut c_void, msg: Box<Box<dyn Any>>) {
    unsafe {
        SendMessageW(hwnd as _, USER_DEF_MSG, 0, Box::into_raw(msg) as _);
    }
}

pub fn send_window_created_msg(hwnd: *mut c_void) {
    unsafe {
        SendMessageW(hwnd as _, WINDOW_CREATED_MSG, 0, 0);
    }
}

pub fn set_window_pos(hwnd: *mut c_void, pos: Point) {
    let (x, y) = pos.into();
    unsafe {
        SetWindowPos(
            hwnd as _,
            HWND_TOP,
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn set_window_size(hwnd: *mut c_void, size: Size) {
    let (w, h) = size.into();
    unsafe {
        SetWindowPos(
            hwnd as _,
            HWND_TOP,
            0,
            0,
            w,
            h,
            SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn enable_maximize_window(hwnd: *mut c_void) {
    unsafe {
        let style = GetWindowLongW(hwnd as _, GWL_STYLE) as u32;
        SetWindowLongW(hwnd as _, GWL_STYLE, (style | WS_MAXIMIZEBOX) as _);
    }
}

pub fn disable_maximize_window(hwnd: *mut c_void) {
    unsafe {
        let style = GetWindowLongW(hwnd as _, GWL_STYLE) as u32;
        SetWindowLongW(hwnd as _, GWL_STYLE, (style & !WS_MAXIMIZEBOX) as _);
    }
}

pub fn fix_window_size(hwnd: *mut c_void) {
    unsafe {
        let style = GetWindowLongW(hwnd as _, GWL_STYLE) as u32;
        SetWindowLongW(hwnd as _, GWL_STYLE, (style & !WS_SIZEBOX) as _);
    }
}

pub fn send_wm_size_message(hwnd: *mut c_void, size: Size) {
    unsafe {
        let (w, h) = size.into();
        SendMessageW(
            hwnd as _,
            WM_SIZE,
            SIZE_RESTORED,
            ((w as u32) | ((h as u32) << 16)) as _,
        );
    }
}

pub fn get_parent_window(hwnd: *mut c_void) -> WindowID {
    unsafe {
        WindowID {
            hwnd: GetParent(hwnd as _) as _,
        }
    }
}
