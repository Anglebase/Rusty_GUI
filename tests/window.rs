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

#[test]
fn ss_demo() {}

use std::time;

struct MyWin {}

impl WinProc for MyWin {
    fn init(&mut self) {
        println!("init");
    }
    fn destroy(&mut self) {
        println!("destroy");
    }
    fn draw(&mut self, g: &mut Graphics) {
        g.full_clear(Color::WHITE);
        let p = Pen::new(PenStyle::Solid, 3, Color::BLACK);
        g.apply_pen(&p);
        let pt = p! {
            g.get_rect().size.width/2,
            g.get_rect().size.height/2,
        };
        let theta = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 1000000;
        let theta = theta as f32 / 1000.0;
        g.line(
            pt,
            pt + p! {
                (theta.sin() * 200.0) as i32,
                (theta.cos() * 200.0) as i32,
            },
        );
        let fs: FontStyle = Default::default();
        let font = Font::new(fs);
        g.apply_font(&font);

        g.text("你好, Rusty GUI!", p! {10,10});
    }
}

#[test]
fn update_test() {
    let w = Window::new(Box::new(MyWin {}), None);
    w.show();
    App::run();
}
