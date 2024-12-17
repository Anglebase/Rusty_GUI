use crate::get_dpi_scale;

pub fn px(x: f32) -> i32 {
    (x * get_dpi_scale()) as i32
}

pub fn em(x: f32) -> i32 {
    px(x * 16.0)
}
