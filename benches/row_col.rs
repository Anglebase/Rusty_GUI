use rusty_gui::{widgets::*, *};

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
fn test() {
    let app = Application::new(true);

    let mut root = Column::new(rect!(100, 100, 1600, 900), None);
    root.as_window().show();

    let mut r1 = Row::new(rect!(0, 0, 100, 100), Some(root.as_window()));
    root.add_layout(r1.as_window().get_id(), LayoutMode::Ratio(2.0));
    r1.as_window().show();

    let mut r2 = Row::new(rect!(0, 0, 100, 100), Some(root.as_window()));
    root.add_layout(r2.as_window().get_id(), LayoutMode::Ratio(1.0));
    r2.as_window().show();

    let test1 = LayoutTest::new(r1.as_window());
    r1.add_layout(test1.as_window().get_id(), LayoutMode::Ratio(2.0));
    test1.as_window().show();

    let test2 = LayoutTest::new(r1.as_window());
    r1.add_layout(
        test2.as_window().get_id(),
        LayoutMode::Range {
            min: Some(100),
            max: Some(500),
            ratio: 1.5,
        },
    );
    test2.as_window().show();

    let test4 = LayoutTest::new(r1.as_window());
    r1.add_layout(test4.as_window().get_id(), LayoutMode::Ratio(2.0));
    test4.as_window().show();

    let test5 = LayoutTest::new(r2.as_window());
    r2.add_layout(test5.as_window().get_id(), LayoutMode::Ratio(1.0));
    test5.as_window().show();

    let test6 = LayoutTest::new(r2.as_window());
    r2.add_layout(test6.as_window().get_id(), LayoutMode::Ratio(2.0));
    test6.as_window().show();

    let test7 = LayoutTest::new(r2.as_window());
    r2.add_layout(test7.as_window().get_id(), LayoutMode::Ratio(1.0));
    test7.as_window().show();

    app.exec(EventLoop::Blocking);
}
