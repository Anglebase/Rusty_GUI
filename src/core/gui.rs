use crate::{Canvas, Event, Window};

pub trait AsWindow {
    fn as_window(&self) -> &Window;
    fn as_window_mut(&mut self) -> &mut Window;
}

pub trait Drawable: AsWindow {
    fn draw(&mut self, canvas: &mut Canvas);
}

pub trait EventListener: AsWindow {
    fn on_event(&mut self, event: &Event);
}

pub trait Ele: Drawable + EventListener {}
impl<T: Drawable + EventListener> Ele for T {}
