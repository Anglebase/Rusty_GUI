use std::{collections::HashMap, i32, sync::mpsc::channel};

use crate::{widgets::LayoutMode, *};

struct Padding {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

pub struct Column {
    this: Window,
    layouts: Vec<(WindowID, LayoutMode)>,
    padding: Padding,
    spacing: i32,
}

impl Default for Column {
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

default_as_window!(Column);

impl Drawable for Column {
    fn draw(&mut self, canvas: &mut Canvas) {
        let _ = canvas;
    }
}

impl EventListener for Column {
    fn on_event(&mut self, event: &Event) {
        if let Event::WindowResized { .. } = event {
            self.update(self.this.rect().size);
        }
    }
}

impl Column {
    fn update(&mut self, size: Size) {
        let size = size!(size.width, size.height);
        // If there is no layout, do nothing
        if self.layouts.is_empty() {
            return;
        }
        // Calculate the minimum and maximum height of the column
        let mut min_height = 0;
        let mut need_max = true;
        let mut max_height = 0;
        for &(_, mode) in &self.layouts {
            match mode {
                LayoutMode::Fixed(height) => {
                    min_height += height;
                }
                LayoutMode::Ratio(_) => {
                    need_max = false;
                }
                LayoutMode::Range { min, max, .. } => {
                    min_height += min.unwrap_or(0);
                    max_height += match max {
                        Some(max) => max,
                        None => {
                            need_max = false;
                            0
                        }
                    };
                }
            }
        }
        // Set the minimum and maximum height of the column window.
        let spaces =
            self.spacing * (self.layouts.len() as i32 - 1) + self.padding.top + self.padding.bottom;
        let ais = self.this.absrect().size.height - self.this.rect().size.height;
        if min_height > 0 {
            self.this.set_min_height(min_height + spaces + ais);
        } else {
            self.this.lift_min_height();
        }
        if need_max {
            self.this.set_max_height(max_height + spaces + ais);
        } else {
            self.this.lift_max_height();
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
                    LayoutMode::Fixed(height) => acc + height,
                    _ => acc,
                }
            });
            // Try to calculate the position and size of each layout.
            let mut result = HashMap::new();
            let mut y = self.padding.top;
            for &(layout, mode) in &test {
                let height = match mode {
                    LayoutMode::Fixed(height) => height,
                    LayoutMode::Ratio(ratio) => ((size.height - spaces - sum_fixed) as f32 * ratio
                        / sum_ratio)
                        .max(0.0) as i32,
                    _ => panic!("Unreachable branch!"),
                };
                let rect = rect!(
                    self.padding.left,
                    y,
                    size.width - self.padding.left - self.padding.right,
                    height
                );
                result.insert(layout, rect);
                y += height + self.spacing;
            }
            // Check the result is in their range and update the test.
            let mut changed = false;
            for (i, &(layout, mode)) in self.layouts.iter().enumerate() {
                if let LayoutMode::Range { min, max, .. } = mode {
                    let height = result.get(&layout).unwrap().size.height;
                    if let Some(min) = min {
                        if height < min {
                            test[i] = (layout, LayoutMode::Fixed(min));
                            changed = true;
                        }
                    }
                    if let Some(max) = max {
                        if height > max {
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
        // Set the width limits of the column window.
        let (tx, rx) = channel();
        self.this.foreach(move |child| {
            let mut it_min_width: Option<i32> = None;
            let mut it_max_width: Option<i32> = None;
            let rect = result.get(&child.as_window().get_id());
            if let Some(rect) = rect {
                child.as_window().set_absrect(*rect);
                if let Some(width) = child.as_window().min_width {
                    if let Some(min_width) = it_min_width {
                        it_min_width = Some(min_width.max(width));
                    } else {
                        it_min_width = Some(width);
                    }
                }
                if let Some(width) = child.as_window().max_width {
                    if let Some(max_width) = it_max_width {
                        it_max_width = Some(max_width.min(width));
                    } else {
                        it_max_width = Some(width);
                    }
                }
            }
            tx.send((it_min_width, it_max_width)).unwrap();
        });
        let (it_min_width, it_max_width) = rx.try_recv().unwrap();
        if let Some(min_width) = it_min_width {
            self.this.set_min_width(min_width);
        } else {
            self.this.lift_min_width();
        }
        if let Some(max_width) = it_max_width {
            self.this.set_max_width(max_width);
        } else {
            self.this.lift_max_width();
        }
        if let Some(id) = self.as_window().parent() {
            id.update_size();
        }
    }
}

impl Column {
    /// Create a new `Column` widget.
    pub fn new(rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        Widget::new("Column", rect, parent)
    }

    /// Add a layout to the column.
    pub fn add_layout(&mut self, layout: WindowID, mode: LayoutMode) {
        self.layouts.push((layout, mode));
        self.update(self.this.rect().size);
    }

    /// Remove a layout from the column.
    pub fn remove_layout(&mut self, layout: &WindowID) {
        self.layouts.retain(|x| x.0 != *layout);
        self.update(self.this.rect().size);
    }

    /// Set the padding of the column.
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

    /// Set the spacing of the column.
    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = spacing;
        self.update(self.this.rect().size);
    }
}
