# Rusty GUI

`rusty_gui` aims to create a low entry threshold Rust GUI framework. It mostly uses a simplified API style, making it more intuitive and simple to use. It is very friendly to Rust's beginners, and you can even learn some programming ideas in Rust that are different from other languages through it. It also can help beginners write graphical programs to enhance their skills.

## Features

+ **Low Entry Threshold**: `rusty_gui` is designed to be easy to use for beginners. It uses a simplified API style, making it more intuitive and simple to use.
+ **Easy to learn and use**: `rusty_gui` is designed to be easy to learn and use. It is very friendly to Rust's beginners, and you can even learn some programming ideas in Rust that are different from other languages through it.
+ **Customizable**: `rusty_gui` is designed to be customizable. You can easily create your own widgets, themes, and other components to fit your needs.

## Usage

To use `rusty_gui`, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_gui = "0.1"
```
Or use the following command to add it to your project:

```
cargo add rusty_gui
```

## Demo

Here is a simple demo of `rusty_gui`:
```rust
use rusty_gui::*;

struct MyWindow {
    this: Window,
    content: String,
}

default_as_window!(MyWindow);

impl Drawable for MyWindow {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(Color::WHITE);
        let style = FontStyle {
            size: 24,
            ..FontStyle::default()
        };
        canvas.set_font(&Font::new(style));
        canvas.rect_text(self.this.rect(), &self.content, TextAlign::Center);
    }
}

impl EventListener for MyWindow {
    fn on_event(&mut self, event: &Event) {
        let _ = event;
    }
}

impl MyWindow {
    fn new(content: &str, rect: Rect) -> Widget<Self> {
        let it = Box::new(Self {
            this: Window::default(),
            content: String::from(content),
        });
        let mut ret = Widget::new(it);
        ret.this = Window::new("MyWindow", rect, None, &ret);
        ret
    }
}

fn main() {
    let app = Application::new(true);

    let window = MyWindow::new("Hello, This is Rusty GUI!", rect!(50, 50, 800, 600));
    window.as_window().show();

    app.exec();
}
```

## License

This library is licensed under the Apache 2.0 license.

## More Information

If you want to learn more about `rusty_gui`, you can read the [learning guide](https://github.com/Anglebase/Rusty_GUI/blob/master/LEARNING.md).
If you want to contribute to `rusty_gui`, you can read the [contribution guide](https://github.com/Anglebase/Rusty_GUI/blob/master/CONTRIBUTING.md).

For more information, please visit the [GitHub repository](https://github.com/Anglebase/Rusty_GUI.git).