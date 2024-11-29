use rusty_gui::*;

#[test]
fn window_creation() {
    struct MyWindow {}
    impl WinProc for MyWindow {
        fn button_down(&self, button: Button) {
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
    let w = Window::new(MyWindow {}, None);
    w.show();
    App::run();
}
