use crate::*;

pub struct Block {
    this: Window,
}

impl Block {
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
