use crate::*;

/// This structrue is the simplest widget's implementation. It is a window with no content.
/// It is a template to implement other widgets.
/// Widget must have a feild of type `Window` and implement `AsWindow` trait.
/// And must implement `Drawable` and `EventListener` trait.
pub struct Block {
    this: Window,
}

impl Block {
    /// Widget's constructor method must return a `Widget<Self>`.
    pub fn new(rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        let mut this = Widget::new(Box::new(Self {
            this: Window::default(),
        }));
        *this.as_window_mut() = Window::new("Block", rect, parent, &this);
        this
    }
}

impl AsWindow for Block {
    fn as_window(&self) -> &Window {
        &self.this
    }
    fn as_window_mut(&mut self) -> &mut Window {
        &mut self.this
    }
}

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
