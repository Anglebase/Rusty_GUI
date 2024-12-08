use std::{any::type_name, ptr::null_mut, sync::Mutex};

use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        ntdef::LPCWSTR,
        windef::{HBRUSH, HWND, RECT},
    },
    um::{libloaderapi::GetModuleHandleW, winuser::*},
};

use crate::{Graph, Rect, Window};

/// Trait `WinProc` defines the behavior for windows.
/// You can implement this trait for your own window types.
/// All methods in this trait have default empty implementations.
#[allow(unused)]
pub trait WinProc {
    fn create(&mut self) {}
    fn destroy(&mut self) {}

    fn draw(&mut self, w: &mut Window, g: &mut Graph) {}

    fn event(&mut self, w: &mut Window) {}
}

static mut WIN_COUNT: Mutex<u32> = Mutex::new(0);
/// Trait `WinImpl` defines interfaces related to window interaction.
/// All types that implement `WinProc` will automatically implement this trait.
/// All methods in this trait needn't be implemented by users.
#[allow(unused)]
pub trait WinImpl: WinProc {
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
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        let it = match this.as_mut() {
            Some(it) => it,
            None => return DefWindowProcW(hwnd, msg, wparam, lparam),
        };
        // Handle the message
        match msg {
            WM_DESTROY => {
                it.destroy();
                // Decrement the window count.
                let count = {
                    let mut count = WIN_COUNT.lock().unwrap();
                    *count -= 1;
                    *count
                };
                // Quit the application when the last window is closed.
                if count == 0 {
                    println!("Quitting");
                    PostQuitMessage(0);
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
                let mut w = Window { hwnd };
                it.draw(&mut w, &mut g);
                EndPaint(hwnd, &ps);
                return 0;
            }
            WM_LBUTTONDOWN => {
                let mut w = Window { hwnd };
                it.event(&mut w);
                return 0;
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    /// Register the window class for the current window type.
    /// If the class has already been registered or registration fails, this function returns `false`.
    /// Otherwise, it returns `true`.
    fn register_this() -> bool {
        let hinstance = unsafe { GetModuleHandleW(null_mut()) };
        // Get type name will be used as the class name.
        let class_name: String = type_name::<Self>().into();
        let class_name: LPCWSTR = class_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr() as _;
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

    /// Create a window with the given title, position, size, and parent window.
    /// If parent is `None`, the window is a top-level window.
    /// Otherwise, the window is a child window.
    fn create_window(&mut self, title: &str, rect: Rect, parent: Option<&Window>) -> Window {
        let title = title
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr() as LPCWSTR;
        // Get the class name.
        let class_name: String = type_name::<Self>().into();
        let class_name: LPCWSTR = class_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr() as _;
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
        self.create();
        // Set the user data to the pointer to the `Self` object.
        unsafe {
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                Box::into_raw(Box::new(Box::new(self))) as _,
            )
        };
        Window { hwnd }
    }
}

impl<T: WinProc> WinImpl for T {}
