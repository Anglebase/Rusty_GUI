# Rusty GUI

This is a simple GUI library for Rust. It only support Windows for now.

*This is the first pre-release version of the library.*

# Features

- **Simple** : The API is designed to be simple and easy to use.

# Usage

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_gui = "0.1.2"
```

Or run `cargo add rusty_gui` in your terminal.

# Example

A simple example of defining and creating a window and showing it:
```rust
// import the module
use rusty_gui::*;

// define a struct for your window
struct MyWindow {
    this: Window,
    // ... other fields you need.
}

// implement the AsWindow trait for your struct
impl AsWindow for MyWindow {
    fn as_window(&self) -> &Window {
        &self.this
    }
    fn as_window_mut(&mut self) -> &mut Window {
        &mut self.this
    }
}

// implement the Drawable trait for your struct
impl Drawable for MyWindow {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(rgb!(120, 173, 255));
        let font = Font::new(FontStyle {
            size: 32,
            ..Default::default()
        });
        canvas.set_font(&font);
        canvas.rect_text(self.as_window().rect(), "Hello, RustyGUI!");
    }
}

// implement the EventListener trait for your struct
impl EventListener for MyWindow {
    fn on_event(&mut self, event: &Event) {
        if let Event::MouseButtonPressed {
            button,
            pos: _,
            mk: _,
        } = event
        {
            if *button == MouseButton::Left {
                println!("You Clicked Me!");
            }
        }
    }
}

impl MyWindow {
    // implement the new method for your struct, It must return a Widget<Self>.
    fn new(rect: Rect) -> Widget<Self> {
        let mut this = Widget::new(Box::new(Self {
            this: Window::default(),
        }));
        *this.as_window_mut() = Window::new("MyWindow", rect, None, &this);
        this
    }
}

fn main() {
    // create an Application object.
    let app = Application::new(true);

    // create and show your window.
    let window = MyWindow::new(rect!(50, 50, 800, 600));
    window.as_window().show();

    // run the application.
    app.exec();
}
```

# License

This project is licensed under the Apache-2.0 license.