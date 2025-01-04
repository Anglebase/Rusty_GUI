mod apis;
mod basic;
mod gdi;
mod winproc;

use std::ptr::null_mut;

pub(crate) use winproc::*;
pub(crate) use apis::*;
pub(crate) use basic::*;
pub(crate) use gdi::*;

pub fn sys_type() -> &'static str {
    unsafe {
        winproc(null_mut(), 0, 0, 0);
    }
    "windows"
}

/// Windows application interface.
pub struct Application;

impl Application {
    /// Initialize the application.
    pub fn new(show_console: bool) -> Self {
        if !show_console {
            close_cmd();
        }
        set_no_auto_dpi_scale();
        Self
    }

    /// Run the application event loop.
    pub fn exec(&self) {
        event_loop();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotKeyFlags {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub win: bool,
}

impl Default for HotKeyFlags {
    fn default() -> Self {
        Self {
            alt: false,
            ctrl: false,
            shift: false,
            win: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowStyle {
    // pub hscroll: bool,
    // pub vscroll: bool,
    pub border: bool,
    pub resize: bool,
    pub caption: bool,
    pub child: bool,
    pub sysmenu: bool,
    pub maxbox: bool,
    pub minbox: bool,
    pub topmost: bool,
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self {
            // hscroll: false,
            // vscroll: false,
            border: false,
            resize: true,
            caption: true,
            child: false,
            sysmenu: true,
            maxbox: true,
            minbox: true,
            topmost: false,
        }
    }
}
