# Rusty GUI

This is a simple GUI library for Rust.

# Usage

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_gui = "0.1.1"
```

Or run `cargo add rusty_gui` in your terminal.

# Example

A simple example of creating a window and showing it:
```rust
use rusty_gui::*; // Import the library

#[derive(Clone)] // It must implement `Clone` trait
struct MyWindow; // Define youself window struct

// Implement the `WinProc` trait for your window struct
impl WinProc for MyWindow {
    fn draw(&mut self, _: &mut rusty_gui::Window, g: &mut rusty_gui::Graph) {
        // It will draw a text on the window at position (100, 100).
        g.text("Hello, Rusty GUI!", p!(100, 100));
    }
}

fn main() {
    // Create your window by yourself window struct.
    let window = MyWindow.create_window("My Window", rect!(200, 200, 800, 600), None);
    // Show the window.
    window.show();
    // Run the event loop.
    App::run();
}

```

You will see a window with the text "Hello, Rusty GUI!" at position (100, 100).

# License

This project is licensed under the Apache-2.0 license.