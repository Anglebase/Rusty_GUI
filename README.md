# Rusty GUI

This is a simple GUI library for Rust.

**It is still in development and may not be fully functional.**

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

A simple example of creating a window and showing it:
```rust
use rusty_gui::*;

fn main() {
    let block = Block::new(rect!(50, 50, 800, 600), None);
    let mut btn = PushButton::new("Button1", rect!(50, 50, 100, 50), block.as_window());

    btn.notifier.add(
        "click",
        Responder::new(|s: &bool| {
            println!("Button clicked: {}", s);
        }),
    );

    block.as_window().show();
    btn.as_window().show();

    exec();
}
```

# License

This project is licensed under the Apache-2.0 license.