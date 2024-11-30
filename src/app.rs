use std::{env::JoinPathsError, ptr::null_mut, sync::Mutex, thread::JoinHandle};

/// This file contains the management interface functions for the program.
/// author: Anglebase (https://github.com/Anglebase)
/// ----------------------------------------------------------------------
use winapi::{
    shared::{
        minwindef::{DWORD, LPVOID},
        windef::*,
    },
    um::{processthreadsapi::CreateThread, winuser::*},
};

use crate::{WinProc, Window};

#[allow(unused)]
pub struct App {
    pub root: Window,
}

impl App {
    pub fn new(wimpl: Box<impl WinProc + 'static>) -> Self {
        let root = Window::new(wimpl, None);
        Self { root }
    }
}

// A part of the WIN32 API functions will casue thread self-locking if these are called in the main thread,
// so this function `th_callback` is used to execute those functions in a separate thread.
static mut FUNC: Mutex<Option<Box<dyn Fn()>>> = Mutex::new(None);
unsafe extern "system" fn th_callback(_: LPVOID) -> DWORD {
    loop {
        let f = {
            let mut buf = FUNC.lock().unwrap();
            let f = buf.take();
            *buf = None;
            f
        };
        if let Some(f) = f {
            f();
        }
    }
}

impl App {
    unsafe fn exec(&mut self) {
        SetProcessDPIAware();
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        // this thread's deadline same as the program's deadline.
        CreateThread(null_mut(), 0, Some(th_callback), null_mut(), 0, null_mut());
        // main thread's message loop
        loop {
            if PeekMessageW(&mut msg, core::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                if msg.message == WM_QUIT {
                    break;
                }
            }
        }
    }

    // this is the interface function to push functions that cannot be executed in the main thread to `th_callback`.
    pub(crate) fn push(f: impl Fn() + 'static) {
        unsafe {
            let mut buf = FUNC.lock().unwrap();
            while let Some(_) = *buf {} // wait
            *buf = Some(Box::new(f));
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            self.exec();
        }
    }
}
