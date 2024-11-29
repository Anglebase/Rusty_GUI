/// Rusty GUI Library
/// author: Anglebase (https://github.com/Anglebase)
/// ------------------------------------------------

mod core;
mod window;
mod graph;
mod math;
mod winproc;
mod app;

pub use core::{
    Window,
    Point,
    Size,
    Rect,
    Graphics,
};
pub use winproc::*;
pub use graph::Color;
pub use app::App;