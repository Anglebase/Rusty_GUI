mod block;
mod pushbutton;
mod switch;
mod lineedit;
mod row;

pub use block::Block;
pub use pushbutton::PushButton;
pub use switch::Switch;
pub use lineedit::LineEdit;
pub use row::Row;

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