use rusty_gui::*;

#[derive(Clone)]
struct MyWin;

impl WinProc for MyWin {
    fn draw(&mut self, this: &mut Window, g: &mut Graph) {
        let mut r = this.rect();
        r.pos = p!(0, 0);
        g.recttext(
            r,
            "This is a simple window in RustyGUI.",
            TextFomat::ATCENTER,
        );
    }

    fn button_down(&mut self, this: &mut Window, button: Button) {
        if let Button::Left(_) = button {
            this.resize(s!(800, 600));
        }
    }

    fn window_resize(&mut self, this: &mut Window, size: Size, size_type: SizeType) {
        info!("Window resized to: {:?}", size);
        info!("Params: this.rect()={:?}", this.rect());
        info!("Params: size_type={:?}", size_type);
    }
}

fn main() {
    set_log_level(LogLevel::Debug);
    let my_window = MyWin.create_window("Simple Window", rect!(100, 100, 800, 600), None);
    my_window.show();
    App::run();
}
