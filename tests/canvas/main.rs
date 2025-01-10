use rusty_gui::{widgets::*, *};

#[test]
fn main() {
    let app = Application::new(true);

    let block = Block::new(rect!(50, 50, 800, 600), None);
    let mut btn = PushButton::new("Button1", rect!(50, 50, 100, 50), block.as_window());

    btn.press.add(
        "click",
        Responder::new(|s: &bool| {
            println!("Button clicked: {}", s);
        }),
    );

    block.as_window().show();
    btn.as_window().show();

    app.exec(EventLoop::Blocking);
}
