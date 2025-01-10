mod apis;
mod basic;
mod gdi;
mod winproc;

use winapi::um::winuser::PostQuitMessage;
pub(crate) use winproc::*;
pub(crate) use apis::*;
pub(crate) use basic::*;
pub(crate) use gdi::*;

/// Windows application interface.
pub struct Application;

/// Event loop mode.
pub enum EventLoop{
    Blocking,
    NonBlocking,
}

impl Application {
    /// Initialize the application.
    pub fn new(show_console: bool) -> Self {
        if !show_console {
            close_cmd();
        }
        set_no_auto_dpi_scale();
        Self
    }

    pub fn exit(&self, exit_code: i32) {
        unsafe {
            PostQuitMessage(exit_code);
        }
    }

    /// Run the application event loop.
    /// This function blocks until the application is closed.
    /// You can specify the event loop mode to either block or non-block.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    /// fn main() {
    ///     let app = Application::new(true);
    ///     // ... other code here
    ///     app.exec(EventLoop::Blocking);
    /// }
    /// ```
    /// The mode of `EventLoop::Blocking` has lower CPU usage than `EventLoop::NonBlocking`.
    pub fn exec(&self, event_loop_mode: EventLoop) {
        event_loop(event_loop_mode);
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
