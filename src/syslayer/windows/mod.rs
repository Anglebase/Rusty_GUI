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
