mod apis;
mod basic;
mod gdi;
mod winproc;

use std::ptr::null_mut;
use winproc::winproc;

pub(crate) use apis::*;
pub(crate) use basic::*;
pub(crate) use gdi::*;

pub fn sys_type() -> &'static str {
    unsafe {
        winproc(null_mut(), 0, 0, 0);
    }
    "windows"
}

pub struct Application;

impl Application {
    pub fn new(show_console: bool) -> Self {
        if !show_console {
            close_cmd();
        }
        set_no_auto_dpi_scale();
        Self
    }

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
