#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use rusty_gui::*;

    struct CanvasBencher {
        this: Window,
        msg: String,
    }

    default_as_window!(CanvasBencher, this);

    impl Default for CanvasBencher {
        fn default() -> Self {
            Self {
                this: Window::default(),
                msg: String::from("Hello, world!"),
            }
        }
    }

    impl CanvasBencher {
        fn new() -> Widget<Self> {
            Widget::new("CanvasBencher", rect!(100, 100, 1600, 900), None)
        }
    }

    impl EventListener for CanvasBencher {
        fn on_event(&mut self, event: &Event) {
            if let Event::WindowResized { size, ty } = event {
                self.msg = format!(
                    "Window resized to {}x{}, type: {:?}",
                    size.width, size.height, ty
                );
                self.this.update();
            }
        }
    }

    impl Drawable for CanvasBencher {
        fn draw(&mut self, canvas: &mut Canvas) {
            canvas.clear(rgb!(50, 100, 150));
            let pen = Pen::new(PenStyle {
                width: 3,
                color: rgb!(255, 0, 0),
                line_style: LineStyle::DashDotDot,
                ..Default::default()
            });
            let brush = Brush::new(rgb!(0, 255, 0));
            let font = Font::new(FontStyle::default());
            canvas.set_font(&font);
            canvas.set_pen(&pen);
            canvas.set_brush(&brush);
            canvas.rect(rect!(5, 5, 50, 50));
            canvas.fill_rect(rect!(65, 5, 50, 50));
            canvas.line(100, 100, 150, 150);
            canvas.polyline(&[pos!(100, 100), pos!(150, 150), pos!(200, 100)]);
            canvas.polygon(&[pos!(200, 100), pos!(250, 150), pos!(300, 100)]);
            canvas.xy_text(pos!(0, 0), &self.msg, Default::default());
            canvas.arc(rect!(100, 100, 150, 100), PI / 2.0, PI);
            canvas.fill_pie(rect!(200, 200, 150, 100), 0.0, PI);
            canvas.pie(rect!(200, 300, 150, 100), 0.0, PI);

            canvas.path(|path| {
                path.move_to((500, 200));
                // Draw a Star:
                let r = rect!(500, 200, 500, 500);
                path.chord(r, PI / 4.0,  PI / 2.0);
                path.circle(r.center(), 1);
                PathShow::Frame
            });
        }
    }

    #[test]
    fn run() {
        let app = Application::new(false);

        let canvas_bencher = CanvasBencher::new();

        canvas_bencher.as_window().show();

        app.exec(EventLoop::Blocking);
    }
}
