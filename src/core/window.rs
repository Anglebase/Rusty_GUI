//! This file is containing the implementation of the Window struct and its methods.

use std::any::Any;
use std::sync::Arc;
use std::{os::raw::c_void, ptr::null_mut};
use crate::*;

use super::{Ele, KeyCode, Widget};

/// The Window struct represents a window on the screen.
/// It is the base of all GUI element.
/// Any GUI element must have a field of type Window. It is used to implement trait `AsWindow`.
#[derive(Clone)]
pub struct Window {
    pub(crate) hwnd: *mut c_void,
    userdata: Option<Arc<dyn Any>>,

}

// It is used to identify the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowID {
    pub(crate) hwnd: *mut c_void,
}

impl Default for Window {
    /// This method returns an empty Window struct as a placeholder.
    fn default() -> Self {
        Self {
            hwnd: null_mut(),
            userdata: None,
        }
    }
}

impl Window {
    fn check_hwnd(&self) {
        if self.hwnd.is_null() {
            panic!("Window cannot be default.");
        }
    }
    
    /// Create a new Window with the given `title`, `rect`, `parent` window, and `widget`.
    /// The parent window can be None if the window is a top-level window.
    /// This function usually does not need to be called by the user, 
    /// because it will be automatically called when you create the widget.
    pub fn new<T: Ele>(title: &str, rect: Rect, parent: Option<&Window>, widget: &Widget<T>) -> Self {
        Self {
            hwnd: create_window(title, rect, parent, widget) as _,
            userdata: None,
        }
    }

    /// Get the area of the window. It is the client rect of window content.
    /// If you want to get the absolute rect of the window relative to the screen, you should use `absrect()` method.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn rect(&self) -> Rect {
        self.check_hwnd();
        get_rect(self.hwnd)
    }

    /// Get the title of the window
    /// # Panics
    /// If the window is default, it will panic.
    pub fn title(&self) -> String {
        self.check_hwnd();
        get_window_title(self.hwnd)
    }

    /// Get the absolute rect of the window relative to the screen, including the title bar and borders.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn absrect(&self) -> Rect {
        self.check_hwnd();
        get_absolute_rect(self.hwnd)
    }

    /// Update the window.
    /// If window needs to be redrawn, this method should be called.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// struct YouWindow {
    ///     this: Window,
    ///     // ...
    ///     data: i32,
    /// }
    ///
    /// default_as_window!(YouWindow);
    ///
    /// impl EventListener for YouWindow{
    ///     fn on_event(&mut self, event: &Event) {
    ///         if let Event::MouseButtonPressed { button, ..} = event {
    ///             // When left button is pressed.
    ///             self.data += 1; // change the data.
    ///             self.this.update(); // update the window content to apply new data.
    ///         }
    ///     }
    /// }
    /// ```
    /// # Panics
    /// If the window is default, it will panic.
    pub fn update(&self) {
        self.check_hwnd();
        update_window(self.hwnd);
    }

    /// Set the rect of the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_rect(&self, rect: Rect) {
        self.check_hwnd();
        set_window_rect(self.hwnd, rect);
    }

    /// Set the position of the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_pos(&self, pos: Point) {
        self.check_hwnd();
        set_window_pos(self.hwnd, pos);
    }

    /// Set the size of the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_size(&self, size: Size) {
        self.check_hwnd();
        set_window_size(self.hwnd, size);
    }

    /// Set the title of the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_title(&self, title: &str) {
        self.check_hwnd();
        set_window_title(self.hwnd, title);
    }

    /// Set the visibility of the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_visible(&self, visible: bool) {
        self.check_hwnd();
        set_window_visible(self.hwnd, visible);
    }

    /// Set the focus to the window.
    /// It will make this window be the target of the Keyboard Event.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_focus(&self) {
        self.check_hwnd();
        set_window_focus(self.hwnd);
    }

    /// Check if the window has focus.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn has_focus(&self) -> bool {
        self.check_hwnd();
        is_window_onfocus(self.hwnd)
    }

    /// Show the window and update it.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn show(&self) {
        self.check_hwnd();
        show_and_update(self.hwnd);
    }

    /// Hide the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn hide(&self) {
        self.check_hwnd();
        self.set_visible(false);
    }

    /// Minimize the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn minimize(&self) {
        self.check_hwnd();
        set_window_minimized(self.hwnd);
    }

    /// Maximize the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn maximize(&self) {
        self.check_hwnd();
        set_window_maximized(self.hwnd);
    }

    /// Restore the window from maximized or minimized state.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn restore(&self) {
        self.check_hwnd();
        set_window_restored(self.hwnd);
    }

    /// Disable the window.
    /// It will cause the window cannot accept any Event.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn disable(&self) {
        self.check_hwnd();
        disable_window(self.hwnd);
    }

    /// Enable the window.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn enable(&self) {
        self.check_hwnd();
        enable_window(self.hwnd);
    }

    /// Register a hotkey for the window.
    /// The `id` is the identifier of the hotkey. It should be between 0 and 0xBFFF.
    /// If the `id` has registered, this function's behavior is undefined.
    /// The `modifiers` and `key` is the hotkey combination.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// let mut block = Block::create(rect!(50,50,800,600), None);
    /// block.as_window_mut().register_hotkey(
    ///     0,
    ///     HotKeyFlags{
    ///         ctrl: true,
    ///         alt: true,
    ///         ..Default::default()
    ///     }, 
    ///     KeyCode::Alpha('L'));   // It will create hotkey 'Ctrl + Alt + L'.
    /// ```
    /// # Panics
    /// If the hotkey `id` is not between 0 and 0xBFFF (Out of range is invalid).
    /// If the window is default, it will panic.
    pub fn register_hotkey(&mut self, id: i32, modifiers: HotKeyFlags, key: KeyCode) {
        if id < 0 || id > 0xBFFF {
            panic!("Invalid hotkey id: {}", id);
        }
        self.check_hwnd();
        register_hotkey_for_window(self.hwnd, id, key, modifiers);
    }

    /// Create a timer for the window.
    /// It will be triggered every `interval` milliseconds.
    /// The `id` is the identifier of the timer. It should be unique for each timer.
    /// If the `id` has used before, it will overwrite the previous timer.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// let block = Block::create(rect!(50,50,800,600), None);
    /// block.as_window().set_timer(0, 1000);   // It will create a timer that triggers every 1000 milliseconds.
    /// ```
    /// # Panics
    /// If the window is default, it will panic.
    pub fn set_timer(&self, id: usize, interval: u32) {
        self.check_hwnd();
        set_window_timer(self.hwnd, id, interval);
    }

    /// Kill a timer for the window.
    /// The `id` is the identifier of the timer. It should be the same as the one used when creating the timer.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// struct YouWindow {
    ///     this: Window,
    /// }
    /// default_as_window!(YouWindow);
    /// 
    /// impl EventListener for YouWindow {
    ///     fn on_event(&mut self, event: &Event) {
    ///         if let Event::Timer { id, .. } = event {
    ///             self.this.kill_timer(*id);   // Kill the timer when it triggers. Make it only trigger once.
    ///             // Do something else.
    ///         }
    ///     }
    /// }
    /// ```
    /// # Panics
    /// If the window is default, it will panic.
    pub fn kill_timer(&self, id: usize) {
        self.check_hwnd();
        kill_window_timer(self.hwnd, id);
    }

    /// For each child window of the window, call the given function.
    /// It will apply the function to each child window recursively.
    /// You can use it with method `read_data()` and `write_data()` to store and retrieve data from the child windows.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// let mut parent = Block::create(rect!(50, 50, 800, 600), None);
    /// let mut child1 = Block::create(rect!(100, 100, 200, 200), None);
    /// let mut child2 = Block::create(rect!(300, 300, 200, 200), None);
    ///
    /// child1.as_window_mut().write_data("Hello, world!".to_string());
    /// child2.as_window_mut().write_data("Goodbye, world!".to_string());
    ///
    /// parent.as_window().foreach(|w: &mut dyn Ele|{
    ///     if let Some(data) = w.as_window().read_data::<String>() {
    ///         // If current window is child1:
    ///         assert_eq!(data, "Hello, world!".to_string());
    ///         // If current window is child2:
    ///         assert_eq!(data, "Goodbye, world!".to_string());
    ///     }
    /// });
    /// ```
    /// # Panics
    /// If the window is default, it will panic.
    pub fn foreach<F: FnMut(&mut dyn Ele) + 'static>(&self, f: F) {
        self.check_hwnd();
        for_each_child_window(self.hwnd, Box::new(f));
    }

    /// Read the data stored in the window.
    /// If `T` is same as the type stored in the window, return the data.
    /// Otherwise, return None.
    /// ```
    /// use rusty_gui::*;
    ///
    /// let parent = Block::create(rect!(50, 50, 800, 600), None);
    /// let mut child = Block::create(rect!(100, 100, 200, 200), None);
    ///
    /// child.as_window_mut().write_data("Hello, world!".to_string());
    ///
    /// parent.as_window().foreach(|w: &mut dyn Ele|{
    ///     if let Some(data) = w.as_window().read_data::<String>() {
    ///         assert_eq!(data, "Hello, world!".to_string());
    ///     }
    /// });
    /// ```
    /// # Panics
    /// If the window is default, it will panic.
    pub fn read_data<T: Clone + 'static>(&self) -> Option<T> {
        self.check_hwnd();
        let userdata = self.userdata.as_ref().map(|d| d.clone());
        userdata.map(|d| -> Option<T> {
            let d = d.downcast_ref::<T>()?;
            Some(d.clone())
        })?
    }

    /// Write the data to the window.
    /// It will overwrite any existing data.
    /// see `read_data()` for example usage.
    /// # Panics
    /// If the window is default, it will panic.
    pub fn write_data<T: Clone + 'static>(&mut self, data: T) {
        self.check_hwnd();
        self.userdata = Some(Arc::new(data));
    }
    
    /// Get the identifier of the window.
    /// It can be used to post message to the window.
    pub fn get_id(&self) -> WindowID {
        self.check_hwnd();
        WindowID { hwnd: self.hwnd }
    }
    
    /// Post a message to the window.
    /// The `msg` is usually an enum of user-defined message.
    /// The `id` is the identifier of the window. It can be obtained by calling `get_id()` method.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// enum MyMessage {
    ///     Quit,
    ///     //...
    /// }
    /// 
    /// let block = Block::create(rect!(50,50,800,600), None);
    /// let id = block.as_window().get_id();
    ///
    /// Window::post(id, Box::new(MyMessage::Quit));
    /// ```
    /// You can handle the message in the `on_message()` method of the window.
    pub fn post<T: Any + 'static>(id: WindowID, msg: T) {
        send_user_def_msg(id.hwnd, Box::new(Box::new(msg)));
    }
}
