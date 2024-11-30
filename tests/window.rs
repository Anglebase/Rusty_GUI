use rusty_gui::*;

#[test]
fn window_creation() {
    struct MyWindow {}
    impl WinProc for MyWindow {
        fn draw(&mut self, g: &mut Graphics) {
            g.full_clear(Color::WHITE);
        }

        fn button_down(&mut self, button: Button) {
            match button {
                Button::Left(point) => {
                    println!("Left button down at {:?}", point);
                }
                Button::Right(point) => {
                    println!("Right button down at {:?}", point);
                }
                Button::Middle(point) => {
                    println!("Middle button down at {:?}", point);
                }
            }
        }
    }
    let w = Window::new(Box::new(MyWindow {}), None);
    w.show();
    App::run();
}
