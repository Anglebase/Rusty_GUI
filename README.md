# Rusty GUI

This is a simple GUI library for Rust. It is still in development and is not yet ready for use.

# Usage

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_gui = "0.1.0"
```

# Example

A simple example of creating a window and showing it:
```rust
use rusty_gui::*;

fn main() { 
    struct MyWin {}

    // `WinProc` is the interface that you need to implement to create a window.
    // You can create a empty impl block to get started.
    // All of its functions have default implementations (Empty implementations).
    impl WinProc for MyWin {
        fn draw(&mut self, g: &mut Graphics) {
            // The window drawing uses double buffering, so the default background is black. 
            // If you think it's harmless, you can skip adding this code.
            g.full_clear(Color::WHITE);
        }
    }

    let window = Window::new(Box::new(MyWin {}), None);
    window.show();
    App::run();
}
```

# License

This project is licensed under the Apache-2.0 license.