mod winproc;
mod basic;
mod apis;
mod gdi;

use std::ptr::null_mut;
use winproc::winproc;

pub(crate) use basic::*;
pub(crate) use apis::*;
pub(crate) use gdi::*;

pub fn sys_type() -> &'static str {
    unsafe {
        winproc(null_mut(), 0, 0, 0);
    }
    "windows"
}
