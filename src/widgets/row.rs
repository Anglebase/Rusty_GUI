use std::{collections::HashMap, i32, sync::mpsc::channel};

use crate::{widgets::LayoutMode, *};

struct Padding {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

/// A container that arranges its children in a row.
/// Its layout logic:
/// - If there are no elements involved in the layout, then nothing will be done.
/// - The Ratio mode element will degrade to Range mode based on its window size range.
/// - The three layout modes are processed according to the following logic:
///     1. LayoutMode::Fixed(width): The element has a fixed width and the width value is width.
///     2. LayoutMode::Ratio(ratio): The element is a proportional width, and the actual width value is allocated based on the proportion value to the total proportion value of all proportional elements, excluding all fixed width elements, margins, and spacing.
///     3. LayoutMode::Range { min, max, ratio }: Firstly, consider it as a Ratio mode element for allocation. If the result is not within the range of min to max, consider it as a Fixed mode element of min or max based on its size until all elements are successfully allocated.
/// - LayoutMode::Ratio(ratio) and LayoutMode::Range { min: None, max: None, ratio } have exactly the same effect.
pub struct Row {
    this: Window,
    layouts: Vec<(WindowID, LayoutMode)>,
    padding: Padding,
    spacing: i32,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            this: Window::default(),
            layouts: vec![],
            padding: Padding {
                left: 5,
                top: 5,
                right: 5,
                bottom: 5,
            },
            spacing: 3,
        }
    }
}

default_as_window!(Row);

impl Drawable for Row {
    fn draw(&mut self, canvas: &mut Canvas) {
        let _ = canvas;
    }
}

impl EventListener for Row {
    fn on_event(&mut self, event: &Event) {
        if let Event::WindowResized { .. } = event {
            self.update(self.this.rect().size);
        }
    }
}

impl Row {
    fn update(&mut self, size: Size) {
        let size = size!(size.width, size.height);
        // If there is no layout, do nothing
        if self.layouts.is_empty() {
            return;
        }
        // Calculate the minimum and maximum width of the row
        let mut min_width = 0;
        let mut need_max = true;
        let mut max_width = 0;
        for &(_, mode) in &self.layouts {
            match mode {
                LayoutMode::Fixed(width) => {
                    min_width += width;
                }
                LayoutMode::Ratio(_) => {
                    need_max = false;
                }
                LayoutMode::Range { min, max, .. } => {
                    min_width += min.unwrap_or(0);
                    max_width += match max {
                        Some(max) => max,
                        None => {
                            need_max = false;
                            0
                        }
                    };
                }
            }
        }
        // Set the minimum and maximum width of the row window.
        let spaces =
            self.spacing * (self.layouts.len() as i32 - 1) + self.padding.left + self.padding.right;
        let ais = self.this.absrect().size.width - self.this.rect().size.width;
        if min_width > 0 {
            self.this.set_min_width(min_width + spaces + ais);
        } else {
            self.this.lift_min_width();
        }
        if need_max {
            self.this.set_max_width(max_width + spaces + ais);
        } else {
            self.this.lift_max_width();
        }
        for (_, mode) in &mut self.layouts {
            let (tx, rx) = channel();
            self.this.foreach(move |child| {
                let min = child.as_window().min_width;
                let max = child.as_window().max_width;
                tx.send((min, max)).unwrap();
            });
            let (min, max) = rx.try_recv().unwrap();
            *mode = if let LayoutMode::Ratio(ratio) = mode {
                LayoutMode::Range {
                    min,
                    max,
                    ratio: *ratio,
                }
            } else {
                *mode
            }
        }
        // Calculate the position and size of each layout
        let mut test = self.layouts.clone();
        test.iter_mut().for_each(|x| {
            let (_, mode) = x;
            if let LayoutMode::Range { ratio, .. } = mode {
                *mode = LayoutMode::Ratio(*ratio);
            }
        });
        let result = loop {
            // Calculate all of the ratios.
            let sum_ratio = test.iter().fold(0.0, |acc, x| {
                let (_, mode) = x;
                match mode {
                    LayoutMode::Ratio(ratio) => acc + *ratio,
                    _ => acc,
                }
            });
            let sum_fixed = test.iter().fold(0, |acc, x| {
                let (_, mode) = x;
                match mode {
                    LayoutMode::Fixed(width) => acc + width,
                    _ => acc,
                }
            });
            // Try to calculate the position and size of each layout.
            let mut result = HashMap::new();
            let mut x = self.padding.left;
            for &(layout, mode) in &test {
                let width = match mode {
                    LayoutMode::Fixed(width) => width,
                    LayoutMode::Ratio(ratio) => ((size.width - spaces - sum_fixed) as f32 * ratio
                        / sum_ratio)
                        .max(0.0) as i32,
                    _ => panic!("Unreachable branch!"),
                };
                let rect = rect!(
                    x,
                    self.padding.top,
                    width,
                    size.height - self.padding.top - self.padding.bottom
                );
                result.insert(layout, rect);
                x += width + self.spacing;
            }
            // Check the result is in their range and update the test.
            let mut changed = false;
            for (i, &(layout, mode)) in self.layouts.iter().enumerate() {
                if let LayoutMode::Range { min, max, .. } = mode {
                    let width = result.get(&layout).unwrap().size.width;
                    if let Some(min) = min {
                        if width < min {
                            test[i] = (layout, LayoutMode::Fixed(min));
                            changed = true;
                        }
                    }
                    if let Some(max) = max {
                        if width > max {
                            test[i] = (layout, LayoutMode::Fixed(max));
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break result;
            }
        };
        // Set the height limits of the row window.
        let (tx, rx) = channel();
        self.this.foreach(move |child| {
            let mut it_min_height: Option<i32> = None;
            let mut it_max_height: Option<i32> = None;
            let rect = result.get(&child.as_window().get_id());
            if let Some(rect) = rect {
                child.as_window().set_absrect(*rect);
                if let Some(height) = child.as_window().min_height {
                    if let Some(min_height) = it_min_height {
                        it_min_height = Some(min_height.max(height));
                    } else {
                        it_min_height = Some(height);
                    }
                }
                if let Some(height) = child.as_window().max_height {
                    if let Some(max_height) = it_max_height {
                        it_max_height = Some(max_height.min(height));
                    } else {
                        it_max_height = Some(height);
                    }
                }
            }
            tx.send((it_min_height, it_max_height)).unwrap();
        });
        let (it_min_height, it_max_height) = rx.try_recv().unwrap();
        if let Some(min_height) = it_min_height {
            self.this.set_min_height(min_height);
        } else {
            self.this.lift_min_height();
        }
        if let Some(max_height) = it_max_height {
            self.this.set_max_height(max_height);
        } else {
            self.this.lift_max_height();
        }
        if let Some(id) = self.as_window().parent() {
            id.update_size();
        }
    }
}

impl Row {
    /// Create a new `Row` widget.
    pub fn new(rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        Widget::new("Row", rect, parent)
    }

    /// Add a layout to the row.
    pub fn add_layout(&mut self, layout: WindowID, mode: LayoutMode) {
        self.layouts.push((layout, mode));
        self.update(self.this.rect().size);
    }

    /// Remove a layout from the row.
    pub fn remove_layout(&mut self, layout: &WindowID) {
        self.layouts.retain(|x| x.0 != *layout);
        self.update(self.this.rect().size);
    }

    /// Set the padding of the row.
    pub fn set_padding(&mut self, padding: i32) {
        self.padding = Padding {
            left: padding,
            top: padding,
            right: padding,
            bottom: padding,
        };
        self.update(self.this.rect().size);
    }

    pub fn set_left_padding(&mut self, padding: i32) {
        self.padding.left = padding;
        self.update(self.this.rect().size);
    }

    pub fn set_top_padding(&mut self, padding: i32) {
        self.padding.top = padding;
        self.update(self.this.rect().size);
    }

    pub fn set_right_padding(&mut self, padding: i32) {
        self.padding.right = padding;
        self.update(self.this.rect().size);
    }

    pub fn set_bottom_padding(&mut self, padding: i32) {
        self.padding.bottom = padding;
        self.update(self.this.rect().size);
    }

    /// Set the spacing of the row.
    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = spacing;
        self.update(self.this.rect().size);
    }
}
