use std::collections::HashMap;

use crate::*;

#[derive(Clone, Copy)]
struct Padding {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[derive(Clone, Copy)]
struct Spacing {
    horizontal: i32,
    vertical: i32,
}

pub struct Grid {
    grid_size: Size,
    this: Window,
    layout: HashMap<WindowID, Rect>,
    padding: Padding,
    spacing: Spacing,
}

default_as_window!(Grid);

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid_size: size!(0, 0),
            this: Window::default(),
            layout: HashMap::new(),
            padding: Padding {
                left: 5,
                top: 5,
                right: 5,
                bottom: 5,
            },
            spacing: Spacing {
                horizontal: 3,
                vertical: 3,
            },
        }
    }
}

impl Drawable for Grid {
    fn draw(&mut self, canvas: &mut Canvas) {
        let _ = canvas;
    }
}

impl EventListener for Grid {
    fn on_event(&mut self, event: &Event) {
        if let Event::WindowResized { size, .. } = event {
            self.update(*size);
        }
    }
}

impl Grid {
    fn update(&mut self, size: Size) {
        if self.grid_size.width == 0 || self.grid_size.height == 0 {
            return;
        }
        let data = self.layout.clone();
        let able_width = size.width
            - self.padding.left
            - self.padding.right
            - (self.spacing.horizontal * (self.grid_size.width - 1));
        let able_height = size.height
            - self.padding.top
            - self.padding.bottom
            - (self.spacing.vertical * (self.grid_size.height - 1));
        let grid_size = self.grid_size;
        let padding = self.padding;
        let spacing = self.spacing;
        let every_width = able_width / grid_size.width;
        let every_height = able_height / grid_size.height;
        self.this.foreach(move |child| {
            let id = child.as_window().get_id();
            if let Some(pos_rect) = data.get(&id) {
                let target_rect = rect!(
                    padding.left + (pos_rect.pos.x * (every_width + spacing.horizontal)),
                    padding.top + (pos_rect.pos.y * (every_height + spacing.vertical)),
                    every_width * pos_rect.size.width
                        + (spacing.horizontal * (pos_rect.size.width - 1)),
                    every_height * pos_rect.size.height
                        + (spacing.vertical * (pos_rect.size.height - 1))
                );
                child.as_window().set_absrect(target_rect);
            }
        });
    }
}

impl Grid {
    pub fn new(rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        Widget::new("Grid", rect, parent)
    }

    pub fn set_padding(&mut self, padding: i32) {
        self.padding = Padding {
            left: padding,
            top: padding,
            right: padding,
            bottom: padding,
        };
        self.update(self.as_window().rect().size);
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = Spacing {
            horizontal: spacing,
            vertical: spacing,
        };
        self.update(self.as_window().rect().size);
    }

    pub fn set_left_padding(&mut self, padding: i32) {
        self.padding.left = padding;
        self.update(self.as_window().rect().size);
    }

    pub fn set_top_padding(&mut self, padding: i32) {
        self.padding.top = padding;
        self.update(self.as_window().rect().size);
    }

    pub fn set_right_padding(&mut self, padding: i32) {
        self.padding.right = padding;
        self.update(self.as_window().rect().size);
    }

    pub fn set_bottom_padding(&mut self, padding: i32) {
        self.padding.bottom = padding;
        self.update(self.as_window().rect().size);
    }

    pub fn set_horizontal_spacing(&mut self, spacing: i32) {
        self.spacing.horizontal = spacing;
        self.update(self.as_window().rect().size);
    }

    pub fn set_vertical_spacing(&mut self, spacing: i32) {
        self.spacing.vertical = spacing;
        self.update(self.as_window().rect().size);
    }

    pub fn add_layout(&mut self, window: WindowID, rect: Rect) {
        let rect = rect!(
            rect.pos.x.min(self.grid_size.width - 1).max(0),
            rect.pos.y.min(self.grid_size.height - 1).max(0),
            rect.size
                .width
                .min(self.grid_size.width - rect.pos.x)
                .max(0),
            rect.size
                .height
                .min(self.grid_size.height - rect.pos.y)
                .max(0)
        );
        self.layout.insert(window, rect);
        self.update(self.as_window().rect().size);
    }

    pub fn remove_layout(&mut self, window: WindowID) {
        self.layout.remove(&window);
        self.update(self.as_window().rect().size);
    }

    pub fn set_size(&mut self, size: Size) {
        self.grid_size = size;
        self.update(self.as_window().rect().size);
    }
}
