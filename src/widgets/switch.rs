use crate::*;

/// demo widget
pub struct Switch {
    this: Window,
    label: String,
    status: bool,
    backcolor: Color,
    pub state_changed: Notifier<bool>,
}

impl Switch {
    pub fn new(label: &str, rect: Rect, parent: &Window) -> Widget<Self> {
        let mut this = Widget::new(Box::new(Self {
            this: Window::default(),
            label: label.to_string().clone(),
            status: false,
            state_changed: Notifier::new(),
            backcolor: rgb!(235),
        }));
        *this.as_window_mut() = Window::new(label, rect, Some(parent), &this);
        this
    }
}

default_as_window!(Switch);

impl Drawable for Switch {
    fn draw(&mut self, canvas: &mut Canvas) {
        let rect = self.as_window().rect();
        canvas.clear(if self.status {
            Color::LIGHT_GRAY
        } else {
            self.backcolor
        });
        let fs = FontStyle {
            size: 24,
            ..Default::default()
        };
        let fo = Font::new(fs);
        canvas.set_font(&fo);
        let text_rect = if self.status {
            Rect {
                pos: rect.pos + Point { x: 1, y: 1 },
                size: rect.size,
            }
        } else {
            rect
        };
        canvas.rect_text(text_rect, &self.label, TextAlign::Center);
        if self.status {
            return;
        }
        let ls = Pen::new(PenStyle {
            width: 2,
            color: Color::DARK_GRAY,
            ..Default::default()
        });
        canvas.set_pen(&ls);
        canvas.line(rect.right(), rect.top(), rect.right(), rect.bottom());
        canvas.line(rect.left(), rect.bottom(), rect.right(), rect.bottom());
    }
}

impl EventListener for Switch {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::MouseButtonPressed {
                button,
                pos: _,
                mk: _,
            } => {
                if *button == MouseButton::Left {
                    self.status = !self.status;
                    self.state_changed.notify(&self.status);
                    self.this.update();
                }
            }
            Event::Hover { pos: _, mk: _ } => {
                self.backcolor = rgb!(215);
                self.this.update();
            }
            Event::Leave => {
                self.backcolor = rgb!(235);
                self.this.update();
            }
            _ => {}
        }
    }
}
