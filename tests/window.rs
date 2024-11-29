use rusty_gui::*;

#[test]
fn window_creation() {
    struct MyWindow {}
    impl WinProc for MyWindow {
        fn draw(&self, graphics: &mut Graphics) {
            graphics.full_clear(Color {
                red: 127,
                green: 127,
                blue: 100,
            });
        }
    }
    let w = Window::new(MyWindow {}, None);
    w.show();
    App::run();
}

#[test]
fn exit_test() {
    struct MyWindow {}
    impl WinProc for MyWindow {
        fn left_button_down(&self, x: i32, y: i32) {
            let _ = (x, y);
            App::exit();
        }
    }

    let w = Window::new(MyWindow {}, None);
    w.show();
    App::run();
}
