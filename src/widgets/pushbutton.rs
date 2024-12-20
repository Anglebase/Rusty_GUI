use crate::*;

/// demo wigdet
pub struct PushButton {
    this: Window,
    label: String,
    status: bool,
    bkcolor: Color,
    pub push: Notifier<bool>,
}

impl PushButton {
    pub fn new(label: &str, rect: Rect, parent: &Window) -> Widget<Self> {
        let mut this = Widget::new(Box::new(Self {
            this: Window::default(),
            label: label.to_string().clone(),
            status: false,
            push: Notifier::new(),
            bkcolor: rgb!(230),
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
        canvas.clear(self.bkcolor);
        let fs = FontStyle {
            size: 16,
            ..Default::default()
        };
        let fo = Font::new(fs);
        canvas.set_font(&fo);
        canvas.rect_text(self.as_window().rect(), &self.label);
    }
}

impl EventListener for PushButton {
    fn on_event(&mut self, event: &crate::Event) {
        match event {
            Event::MouseButtonPressed {
                button,
                pos: _,
                mk: _,
            } => {
                if *button == MouseButton::Left {
                    self.status = true;
                    self.push.notify(&self.status);
                }
            }
            Event::MouseButtonReleased {
                button,
                pos: _,
                mk: _,
            } => {
                if *button == MouseButton::Left {
                    self.status = false;
                    self.push.notify(&self.status);
                }
            }
            Event::Hover { pos: _, mk: _ } => {
                self.bkcolor = rgb!(200);
                self.this.update();
            }
            Event::Leave => {
                self.bkcolor = rgb!(230);
                self.this.update();
            }
            _ => {}
        }
    }
}
