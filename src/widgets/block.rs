use crate::*;

/// This structure is the simplest widget's implementation. It is a window with no content.
/// It is a template to implement other widgets.
/// Widget must have a field of type `Window` and implement `AsWindow` trait.
/// And must implement `Drawable` and `EventListener` trait.
pub struct Block {
    this: Window,
}

impl Block {
    /// Create a new instance of `Block` struct.
    pub fn new() -> Self {
        Self {
            this: Window::default(),
        }
    }
    /// Widget's constructor method must return a `Widget<Self>`.
    pub fn create(rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        Widget::new("Block", rect, parent, Self::new())
    }
}

default_as_window!(Block);

impl Drawable for Block {
    fn draw(&mut self, canvas: &mut crate::Canvas) {
        let _ = canvas;
    }
}

impl EventListener for Block {
    fn on_event(&mut self, event: &crate::Event) {
        let _ = event;
    }
}
