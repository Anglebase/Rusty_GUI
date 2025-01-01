use crate::*;

const BORDER_WIDTH: i32 = 5;

/// This is a simple line edit widget that allows the user to input text.
/// It without cursor or selection, and user can only append or delete characters at the end of the text.
/// And it can't scroll even if the text is too long to fit in the widget.
pub struct LineEdit {
    this: Window,
    content: String,
    placeholder: String,
    pub content_changed: Notifier<String>,
    pub enter: Notifier<String>,

    border_pen: Pen,
    frame_pen: Pen,
    text_color: Color,
    placeholder_color: Color,
    pub text: Font,
}

default_as_window!(LineEdit);

impl Drawable for LineEdit {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(Color::WHITE);
        let rect = self.this.rect();

        let text_rect = {
            let size = rect.size - size!(2 * BORDER_WIDTH, 2 * BORDER_WIDTH);
            rect.center_rect(size)
        };
        canvas.set_font(&self.text);
        if self.content.is_empty() {
            canvas.set_text_color(self.placeholder_color);
            canvas.rect_text(text_rect, &self.placeholder, TextAlign::LeftMiddle);
        } else {
            canvas.set_text_color(self.text_color);
            canvas.rect_text(text_rect, &self.content, TextAlign::LeftMiddle);
        }
        canvas.set_pen(&self.border_pen);
        canvas.rect(rect);
        canvas.set_pen(&self.frame_pen);
        canvas.rect(rect);
    }
}

impl EventListener for LineEdit {
    fn on_event(&mut self, event: &Event) {
        if let Event::MouseButtonPressed { button, .. } = event {
            if *button == MouseButton::Left {
                self.this.set_focus();
            }
            self.this.update();
        }
        if let Event::Input { ch: char } = event {
            match char {
                '\x08' => {
                    if !self.content.is_empty() {
                        self.content.pop();
                    }
                }
                '\r' | '\n' => {
                    let content = self.content.clone();
                    println!("Enter pressed");
                    self.enter.notify(&content);
                    return;
                }
                '\t' => {
                    self.content.push(' ');
                }
                _ => {
                    self.content.push(*char);
                }
            }
            let content = self.content.clone();
            self.content_changed.notify(&content);
            self.this.update();
        }
    }
}

impl LineEdit {
    pub fn new(placeholder: &str, rect: Rect, parent: &Window) -> Widget<LineEdit> {
        let frame_pen_style = PenStyle {
            width: 2,
            ..Default::default()
        };
        let border_pen_style = PenStyle {
            width: BORDER_WIDTH as u32,
            color: Color::WHITE,
            ..Default::default()
        };
        let text_font = FontStyle {
            size: 24,
            ..Default::default()
        };

        let it = Box::new(Self {
            this: Window::default(),
            content: String::new(),
            placeholder: String::from(placeholder),
            content_changed: Notifier::new(),
            enter: Notifier::new(),

            frame_pen: Pen::new(frame_pen_style),
            border_pen: Pen::new(border_pen_style),
            text_color: Color::BLACK,
            placeholder_color: Color::GRAY,
            text: Font::new(text_font),
        });
        let mut this = Widget::new(it);
        this.this = Window::new("LineEdit", rect, Some(parent), &this);
        this
    }
}
