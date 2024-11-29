# Rusty GUI

This is a simple GUI library for Rust. It is still in development and is not yet ready for use.

# Usage

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_gui = "0.1.0"
```

# Example

```rust
use rusty_gui::*;

fn main() {
    struct MyWindow {}
    impl WinProc for MyWindow {
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
    let w = Window::new(MyWindow {}, None);
    w.show();
    App::run();
}
```