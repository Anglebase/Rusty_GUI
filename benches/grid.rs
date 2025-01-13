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
fn test_layout() {
    let app = Application::new(true);

    let mut grid = Grid::new(rect!(100, 100, 1000, 600), None);
    grid.set_size(size!(3, 3));

    let test1 = LayoutTest::new(grid.as_window());
    grid.add_layout(test1.as_window().get_id(), rect!(0, 0, 1, 2));
    test1.as_window().show();

    let test2 = LayoutTest::new(grid.as_window());
    grid.add_layout(test2.as_window().get_id(), rect!(1, 0, 2, 1));
    test2.as_window().show();

    let mut test3 = LayoutTest::new(grid.as_window());
    grid.add_layout(test3.as_window().get_id(), rect!(1, 1, 1, 1));
    test3.as_window().show();

    let test4 = LayoutTest::new(grid.as_window());
    grid.add_layout(test4.as_window().get_id(), rect!(2, 1, 1, 2));
    test4.as_window().show();

    let test5 = LayoutTest::new(grid.as_window());
    grid.add_layout(test5.as_window().get_id(), rect!(0, 2, 2, 1));
    test5.as_window().show();

    grid.as_window().show();
    app.exec(EventLoop::Blocking);
}
