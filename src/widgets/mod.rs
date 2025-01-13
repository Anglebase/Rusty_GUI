mod block;
mod pushbutton;
mod switch;
mod lineedit;
mod row;
mod column;
mod grid;

pub use block::Block;
pub use pushbutton::PushButton;
pub use switch::Switch;
pub use lineedit::LineEdit;
pub use row::Row;
pub use column::Column;
pub use grid::Grid;

#[derive(Debug, Clone, Copy)]
pub enum LayoutMode {
    Fixed(i32),
    Ratio(f32),
    Range{
        min: Option<i32>,
        max: Option<i32>,
        ratio: f32,
    },
}