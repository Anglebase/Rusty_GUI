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

/// Layout mode for Row and Column widgets.
#[derive(Debug, Clone, Copy)]
pub enum LayoutMode {
    /// Fixed number of pixels.
    Fixed(i32),
    /// Ratio of the remaining space.
    Ratio(f32),
    /// Value range constraint ratio.
    Range{
        /// Minimum value.
        min: Option<i32>,
        /// Maximum value.
        max: Option<i32>,
        /// Ratio of the remaining space.
        ratio: f32,
    },
}