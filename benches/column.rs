use rusty_gui::*;
use widgets::{Column, LayoutMode};

struct LayoutTest {
    this: Window,
}

default_as_window!(LayoutTest);

impl Default for LayoutTest {
    fn default() -> Self {
        Self {
            this: Window::default(),
        }
    }
}

impl Drawable for LayoutTest {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(Color::GRAY);
    }
}

impl EventListener for LayoutTest {
    fn on_event(&mut self, event: &Event) {
        let _ = event;
    }
}

impl LayoutTest {
    fn new(parent: &Window) -> Widget<Self> {
        Widget::new("LayoutTest", rect!(0, 0, 100, 100), Some(parent))
    }
}

#[test]
fn test_layout() {
    let app = Application::new(true);

    let mut root = Column::new(rect!(100, 100, 1000, 600), None);
    root.as_window().show();

    let test1 = LayoutTest::new(root.as_window());
    root.add_layout(test1.as_window().get_id(), LayoutMode::Fixed(200));
    test1.as_window().show();

    let test2 = LayoutTest::new(root.as_window());
    root.add_layout(test2.as_window().get_id(), LayoutMode::Ratio(2.0));
    test2.as_window().show();

    let test3 = LayoutTest::new(root.as_window());
    root.add_layout(
        test3.as_window().get_id(),
        LayoutMode::Range {
            min: Some(150),
            max: Some(400),
            ratio: 2.0,
        },
    );
    test3.as_window().show();

    let test4 = LayoutTest::new(root.as_window());
    root.add_layout(
        test4.as_window().get_id(),
        LayoutMode::Range {
            min: Some(200),
            max: Some(300),
            ratio: 4.0,
        },
    );
    test4.as_window().show();

    app.exec(EventLoop::Blocking);
}
