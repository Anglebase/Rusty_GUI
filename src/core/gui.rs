use crate::{Canvas, Event, Window};

pub trait Userdata {
    fn as_any(&self) -> Option<&dyn std::any::Any>;
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any>;
}

#[macro_export]
macro_rules! default_userdata {
    ($def_type:ty) => {
        impl Userdata for $def_type {
            fn as_any(&self) -> Option<&dyn std::any::Any> {
                None
            }
            fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
                None
            }
        }
    };
}

pub trait AsWindow: Userdata {
    fn as_window(&self) -> &Window;
    fn as_window_mut(&mut self) -> &mut Window;
}

#[macro_export]
macro_rules! default_aswindow {
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
}

pub trait Drawable: AsWindow {
    fn draw(&mut self, canvas: &mut Canvas);
}

pub trait EventListener: AsWindow {
    fn on_event(&mut self, event: &Event);
}

pub trait Ele: Drawable + EventListener {}
impl<T: Drawable + EventListener> Ele for T {}
