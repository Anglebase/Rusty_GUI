//! This file is containing the implementation of the Window struct and its methods.

use std::any::Any;
use std::sync::Arc;
use std::{os::raw::c_void, ptr::null_mut};
use winapi::um::winuser::EnumChildWindows;

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
    /// Create a new Window with the given `title`, `rect`, `parent` window, and `widget`.
    /// The parent window can be None if the window is a top-level window.
    /// The `widget` should be the wrapper of the structure instance that holds this window and implements the trait `Ele`.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    /// 
    /// struct YouWindow {
    ///     this: Window,
    /// }
    /// 
    /// // ...
    /// // Add other code to implement the `Ele` trait.
    /// // ...
    /// 
    /// impl YouWindow {
    ///     // Define `Ele` structure specific constructor. It doesn't return by its type, it returns a wrapper.
    ///     fn new(title: &str, area: Rect, parent: Option<&Window>) -> Widget<Self> {
    ///         let object = Box::new(Self {
    ///             this: Window::default(),    // use `Window::default()` to occupy space.
    ///         });
    ///         let mut result = Widget::new(object);
    ///         result.this = Window::new(title, area, parent, &result);    // It needs the instance of Widget. 
    ///         result
    ///     }
    /// }
    /// ```
    pub fn new<T: Ele>(title: &str, rect: Rect, parent: Option<&Window>, widget: &Widget<T>) -> Self {
        Self {
            hwnd: create_window(title, rect, parent, widget) as _,
            userdata: None,
        }
    }

    /// Get the area of the window. It is the client rect of window content.
    /// If you want to get the absolute rect of the window relative to the screen, you should use `absrect()` method.
    pub fn rect(&self) -> Rect {
        get_rect(self.hwnd)
    }

    /// Get the title of the window
    pub fn title(&self) -> String {
        get_window_title(self.hwnd)
    }

    /// Get the absolute rect of the window relative to the screen, including the title bar and borders.
    pub fn absrect(&self) -> Rect {
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
    pub fn update(&self) {
        update_window(self.hwnd);
    }

    /// Set the rect of the window.
    pub fn set_rect(&self, rect: Rect) {
        set_window_rect(self.hwnd, rect);
    }

    /// Set the title of the window.
    pub fn set_title(&self, title: &str) {
        set_window_title(self.hwnd, title);
    }

    /// Set the visibility of the window.
    pub fn set_visible(&self, visible: bool) {
        set_window_visible(self.hwnd, visible);
    }

    /// Set the focus to the window.
    /// It will make this window be the target of the Keyboard Event.
    pub fn set_focus(&self) {
        set_window_focus(self.hwnd);
    }

    /// Check if the window has focus.
    pub fn has_focus(&self) -> bool {
        is_window_onfocus(self.hwnd)
    }

    /// Show the window and update it.
    pub fn show(&self) {
        show_and_update(self.hwnd);
    }

    /// Hide the window.
    pub fn hide(&self) {
        self.set_visible(false);
    }

    /// Minimize the window.
    pub fn minimize(&self) {
        set_window_minimized(self.hwnd);
    }

    /// Maximize the window.
    pub fn maximize(&self) {
        set_window_maximized(self.hwnd);
    }

    /// Restore the window from maximized or minimized state.
    pub fn restore(&self) {
        set_window_restored(self.hwnd);
    }

    /// Disable the window.
    /// It will cause the window cannot accept any Event.
    pub fn disable(&self) {
        disable_window(self.hwnd);
    }

    /// Enable the window.
    pub fn enable(&self) {
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
    /// let block = Block::new(rect!(50,50,800,600), None);
    /// block.as_window().register_hotkey(
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
    pub fn register_hotkey(&mut self, id: i32, modifiers: HotKeyFlags, key: KeyCode) {
        if id < 0 || id > 0xBFFF {
            panic!("Invalid hotkey id: {}", id);
        }
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
    /// let block = Block::new(rect!(50,50,800,600), None);
    /// block.as_window().set_timer(0, 1000);   // It will create a timer that triggers every 1000 milliseconds.
    /// ```
    pub fn set_timer(&self, id: usize, interval: u32) {
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
    pub fn kill_timer(&self, id: usize) {
        kill_window_timer(self.hwnd, id);
    }

    /// For each child window of the window, call the given function.
    /// It will apply the function to each child window recursively.
    /// You can use it with method `read_data()` and `write_data()` to store and retrieve data from the child windows.
    /// # Example
    /// ```
    /// use rusty_gui::*;
    ///
    /// let mut parent = Block::new(rect!(50, 50, 800, 600), None);
    /// let mut child1 = Block::new(rect!(100, 100, 200, 200), None);
    /// let mut child2 = Block::new(rect!(300, 300, 200, 200), None);
    ///
    /// child1.as_window_mut().write_data("Hello, world!".to_string());
    /// child2.as_window_mut().write_data("Goodbye, world!".to_string());
    ///
    /// parent.as_window().foreach(Box::new(|w|{
    ///     if let Some(data) = w.as_window().read_data::<String>() {
    ///         // If current window is child1:
    ///         assert_eq!(data, "Hello, world!".to_string());
    ///         // If current window is child2:
    ///         assert_eq!(data, "Goodbye, world!".to_string());
    ///     }
    /// }));
    /// ```
    pub fn foreach(&self, mut f: Box<dyn FnMut(&mut dyn Ele)>) {
        unsafe {
            EnumChildWindows(
                self.hwnd as _,
                Some(enum_windows_callback),
                &mut f as *mut _ as _,
            );
        }
    }

    /// Read the data stored in the window.
    /// If `T` is same as the type stored in the window, return the data.
    /// Otherwise, return None.
    /// ```
    /// use rusty_gui::*;
    ///
    /// let parent = Block::new(rect!(50, 50, 800, 600), None);
    /// let mut child = Block::new(rect!(100, 100, 200, 200), None);
    ///
    /// child.as_window_mut().write_data("Hello, world!".to_string());
    ///
    /// parent.as_window().foreach(Box::new(|w|{
    ///     if let Some(data) = w.as_window().read_data::<String>() {
    ///         assert_eq!(data, "Hello, world!".to_string());
    ///     }
    /// }));
    /// ```
    pub fn read_data<T: Clone + 'static>(&self) -> Option<T> {
        let userdata = self.userdata.as_ref().map(|d| d.clone());
        userdata.map(|d| -> Option<T> {
            let d = d.downcast_ref::<T>()?;
            Some(d.clone())
        })?
    }

    /// Write the data to the window.
    /// It will overwrite any existing data.
    /// see `read_data()` for example usage.
    pub fn write_data<T: Clone + 'static>(&mut self, data: T) {
        self.userdata = Some(Arc::new(data));
    }
}
