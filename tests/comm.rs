use std::fmt::Debug;

use rusty_gui::*;

#[derive(Clone)]
struct PushButton {
    content: String,
    status: bool,
}

impl WinProc for PushButton {
    fn draw(&mut self, this: &mut Window, g: &mut Graph) {
        let mut rect = this.rect();
        rect.pos = p!(0, 0);
        g.recttext(rect, &self.content, TextFomat::ATCENTER);
    }

    fn button_down(&mut self, this: &mut Window, button: Button) {
        if let Button::Left(_) = button {
            info!("Button clicked: {}", self.content);
        }
    }
}
#[derive(Clone)]
struct Accept {
    content: String,
}

impl WinProc for Accept {
    fn draw(&mut self, this: &mut Window, g: &mut Graph) {
        let mut rect = this.rect();
        rect.pos = p!(0, 0);
        g.recttext(rect, &self.content, TextFomat::ATCENTER);
    }
}

#[derive(Clone)]
struct MainWindow;

impl WinProc for MainWindow {}

fn main() {
    set_log_level(LogLevel::Debug);
    let mainwindow = MainWindow.create_window("Test Window", rect!(200, 200, 800, 600), None);
    let a = Accept {
        content: "Accept".to_string(),
    }
    .create_window("", rect!(10, 10, 100, 50), Some(&mainwindow));
    let b = PushButton {
        content: "Button 1".to_string(),
        status: false,
    }
    .create_window("", rect!(10, 70, 100, 50), Some(&mainwindow));

    mainwindow.show();
    a.show();
    b.show();

    App::run();
}
