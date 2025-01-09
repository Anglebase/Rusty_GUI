//! This file contains the core GUI requirements and their traits.

use std::any::Any;
use crate::{Canvas, Event, Window};

/// A trait for elements that can be used as a `Window`.
pub trait AsWindow {
    fn as_window(&self) -> &Window;
    fn as_window_mut(&mut self) -> &mut Window;
}

/// A macro for defining a default implementation of the `AsWindow` trait for a given type.
/// This implementation returns a reference to the given window name for both `as_window` and `as_window_mut` methods.
/// If your type has a field named `this` of type `Window`, you can simply call the macro without arguments.
/// ## Example
/// ```
/// use rusty_gui::*;
/// struct MyType { this: Window }
/// default_as_window!(MyType);
/// ```
/// If your type has a different name for the window, you can pass it as an argument to the macro.
/// ## Example
/// ```
/// use rusty_gui::*;
/// struct MyType { my_window: Window }
/// default_as_window!(MyType, my_window);
/// ```
/// If your type has not field of type `Window`, you should implement the `AsWindow` trait by hand.
#[macro_export]
macro_rules! default_as_window {
    ($def_type:ty, $window_name:ident) => {
        impl AsWindow for $def_type {
            fn as_window(&self) -> &Window {
                &self.$window_name
            }
            fn as_window_mut(&mut self) -> &mut Window {
                &mut self.$window_name
            }
        }
    };
    ($def_type:ty) => {
        impl AsWindow for $def_type {
            fn as_window(&self) -> &Window {
                &self.this
            }
            fn as_window_mut(&mut self) -> &mut Window {
                &mut self.this
            }
        }
    };
}

/// A trait for elements that can be drawn on a `Canvas`.
pub trait Drawable: AsWindow {
    fn draw(&mut self, canvas: &mut Canvas);
}

/// A trait for elements that can listen to events.
pub trait EventListener: AsWindow {
    fn on_event(&mut self, event: &Event);
    fn on_message(&mut self, msg: Box<dyn Any>){
        let _ = msg;
    }
}

/// Automatically implements this trait for any type that implements `Drawable` and `EventListener`.
pub trait AbstractElement: Drawable + EventListener {}
impl<T: Drawable + EventListener> AbstractElement for T {}

pub trait Element: AbstractElement + Default {}
impl<T: AbstractElement + Default> Element for T {}