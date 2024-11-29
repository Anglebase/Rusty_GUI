use rusty_gui::*;

struct MyWin {

}

impl WinProc for MyWin {
    fn draw(&self, graphics: &mut Graphics) {
        graphics.full_clear(Color::new(
            255,
            127,
            255,
        ));
    }
}

fn main() {
    let w = Window::new(MyWin {}, None);
    w.show();
    App::run();
}
