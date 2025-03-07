use rusty_gui::*;

#[test]
fn main() {
    let app = Application::new(true);

    let block = Block::create(rect!(50, 50, 800, 600), None);
    let mut btn = PushButton::create("Button1", rect!(50, 50, 100, 50), block.as_window());

    btn.press.add(
        "click",
        Responder::new(|s: &bool| {
            println!("Button clicked: {}", s);
        }),
    );

    block.as_window().show();
    btn.as_window().show();

    app.exec();
}
