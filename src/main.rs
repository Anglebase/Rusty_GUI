use rusty_gui::*;

struct MyWin;

impl WinProc for MyWin {
    fn create(&mut self) {
        println!("Creating MyWin");
    }
    fn destroy(&mut self) {
        println!("Destroying MyWin");
    }

    fn draw(&mut self, _: &mut Window, g: &mut Graph) {
        let p = Pen::new(PenStyle::Solid, 5, Color::RED);
        g.apply_pen(&p);
        g.line(Point { x: 10, y: 10 }, Point { x: 100, y: 100 });
        g.text("Hello, Rusty GUI!", Point { x: 10, y: 10 });

        println!("Drawing MyWin");
    }

    fn event(&mut self, w: &mut Window) {
        w.update();
    }
}

fn main() {
    MyWin::register_this();
    let mut rect = Rect {
        pos: Point { x: 100, y: 100 },
        size: Size {
            width: 800,
            height: 600,
        },
    };
    let w = MyWin.create_window("My Window", rect, None);
    rect.pos = Point { x: 200, y: 200 };
    let w2 = MyWin.create_window("My Window 2", rect, None);
    w.show();
    w2.show();

    w2.set_parent(&w);

    App::run();
}
