use crate::{Canvas, Event, Window};

/// A trait for elements that can be used as a `Window`.
pub trait AsWindow {
    fn as_window(&self) -> &Window;
    fn as_window_mut(&mut self) -> &mut Window;
}

/// A macro for defining a default implementation of the `AsWindow` trait for a given type.
/// This implementation returns a reference to the given window name for both `as_window` and `as_window_mut` methods.
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
}

/// Automatically implements this trait for any type that implements `Drawable` and `EventListener`.
pub trait Ele: Drawable + EventListener {}
impl<T: Drawable + EventListener> Ele for T {}
