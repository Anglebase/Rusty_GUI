use std::os::raw::c_void;

use crate::{
    clear_device, delete_object, draw_arc, draw_circle, draw_ellipse, draw_fill_polygon, draw_fill_rect, draw_fille_circle, draw_fille_ellipse, draw_fille_pie, draw_line, draw_pie, draw_polygon, draw_polyline, draw_rect, draw_rect_text, draw_xy_text, new_brush_object, new_pen_object, select_object, BrushParam, Color, PenParam, Point, Rect
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
        draw_rect(self.hdc, rect);
    }

    pub fn polyline(&self, points: &[Point]) {
        draw_polyline(self.hdc, points);
    }

    pub fn polygon(&self, points: &[Point]) {
        draw_polygon(self.hdc, points);
    }

    pub fn arc(&self, rect: Rect, start: f32, sweep: f32) {
        draw_arc(self.hdc, rect, start, sweep);
    }

    pub fn pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_pie(self.hdc, rect, start, sweep);
    }

    pub fn ellipse(&self, rect: Rect) {
        draw_ellipse(self.hdc, rect);
    }

    pub fn circle(&self, pos: Point, radius: i32) {
        draw_circle(self.hdc, pos, radius);
    }

    pub fn fill_rect(&self, rect: Rect) {
        draw_fill_rect(self.hdc, rect);
    }

    pub fn fill_polygon(&self, points: &[Point]) {
        draw_fill_polygon(self.hdc, points);
    }

    pub fn fill_pie(&self, rect: Rect, start: f32, sweep: f32) {
        draw_fille_pie(self.hdc, rect, start, sweep);
    }

    pub fn fill_ellipse(&self, rect: Rect) {
        draw_fille_ellipse(self.hdc, rect);
    }

    pub fn fill_circle(&self, pos: Point, radius: i32) {
        draw_fille_circle(self.hdc, pos, radius);
    }

    pub fn xytext(&self, pos: Point, text: &str) {
        draw_xy_text(self.hdc, pos, text);
    }

    pub fn rect_text(&self, rect: Rect, text: &str) {
        draw_rect_text(self.hdc, rect, text);
    }
}
