use crate::{
    AsWindow, Color, Drawable, Event, EventListener, MouseButton, Notifier, Rect, Widget, Window,
};

pub struct PushButton {
    this: Window,
    label: String,
    status: bool,
    pub notifier: Notifier<bool>,
}

impl PushButton {
    pub fn new(label: &str, rect: Rect, parent: &Window) -> Widget<Self> {
        let mut this = Widget::new(Box::new(Self {
            this: Window::default(),
            label: label.to_string().clone(),
            status: false,
            notifier: Notifier::new(),
        }));
        *this.as_window_mut() = Window::new(label, rect, Some(parent), &this);
        this
    }
}

impl AsWindow for PushButton {
    fn as_window(&self) -> &Window {
        &self.this
    }

    fn as_window_mut(&mut self) -> &mut Window {
        &mut self.this
    }
}

impl Drawable for PushButton {
    fn draw(&mut self, canvas: &mut crate::Canvas) {
        canvas.clear(Color::RED);
        println!("Draw PushButton: {}", self.label);
    }
}

impl EventListener for PushButton {
    fn on_event(&mut self, event: &crate::Event) {
        if let Event::MouseButtonPressed {
            button,
            pos: _,
            mk: _,
        } = event
        {
            if *button == MouseButton::Left {
                self.status = true;
                self.notifier.notify(&self.status);
            }
        }
        if let Event::MouseButtonReleased {
            button,
            pos: _,
            mk: _,
        } = event
        {
            if *button == MouseButton::Left {
                self.status = false;
                self.notifier.notify(&self.status);
            }
        }
    }
}
