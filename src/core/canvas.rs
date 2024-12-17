use std::os::raw::c_void;

use crate::{
    clear_device, delete_object, draw_line, new_brush_object, new_pen_object, select_object,
    BrushParam, Color, PenParam, Rect,
};

pub enum PenStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
}

#[derive(Clone)]
pub struct Pen {
    hpen: *mut c_void,
}
impl Drop for Pen {
    fn drop(&mut self) {
        delete_object(self.hpen);
    }
}
impl Pen {
    pub fn new(style: PenStyle, width: i32, color: Color) -> Self {
        Self {
            hpen: new_pen_object(PenParam::Normal(style, width, color)),
        }
    }
}

#[derive(Clone)]
pub struct Brush {
    hbrush: *mut c_void,
}
impl Drop for Brush {
    fn drop(&mut self) {
        delete_object(self.hbrush);
    }
}
impl Brush {
    pub fn new(color: Color) -> Self {
        Self {
            hbrush: new_brush_object(BrushParam::Solid(color)),
        }
    }
}

#[derive(Clone)]
pub struct Font {
    hfont: *mut c_void,
}
impl Drop for Font {
    fn drop(&mut self) {
        delete_object(self.hfont);
    }
}

pub struct Canvas {
    pub(crate) hdc: *mut c_void,
    pub(crate) rect: Rect,
}

impl Canvas {
    pub fn clear(&self, color: Color) {
        clear_device(self.hdc, self.rect, color);
    }

    pub fn set_pen(&self, pen: &Pen) -> Pen {
        Pen {
            hpen: select_object(self.hdc, pen.hpen) as *mut c_void,
        }
    }

    pub fn set_brush(&self, brush: &Brush) -> Brush {
        Brush {
            hbrush: select_object(self.hdc, brush.hbrush) as *mut c_void,
        }
    }

    pub fn set_font(&self, font: &Font) -> Font {
        Font {
            hfont: select_object(self.hdc, font.hfont) as *mut c_void,
        }
    }

    pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        draw_line(self.hdc, x1, y1, x2, y2);
    }

    pub fn rect(&self, rect: Rect) {
        let (x, y, w, h) = rect.into();
        self.line(x, y, x + w, y);
        self.line(x + w, y, x + w, y + h);
        self.line(x + w, y + h, x, y + h);
        self.line(x, y + h, x, y);
    }
}
