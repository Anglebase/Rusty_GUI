use crate::*;
use std::os::raw::c_void;

pub struct BitMap {
    pub(crate) bmp: *mut c_void,
    pub(crate) mdc: *mut c_void,
    pub(crate) size: Size,
}

impl Drop for BitMap {
    fn drop(&mut self) {
        delete_object(self.bmp);
        delete_hdc(self.mdc);
    }
}

impl BitMap {
    pub fn new_from_canvas(canvas: &Canvas) -> Self {
        let size = canvas.rect.size;
        BitMap {
            bmp: new_bitmap(canvas.hdc, size.width, size.height),
            mdc: new_hdc(canvas.hdc),
            size,
        }
    }

    pub fn canvas<F: FnMut(&mut Canvas)>(&self, mut f: F) {
        select_object(self.mdc, self.bmp as _);
        let mut canvas: Canvas = Canvas {
            hdc: self.mdc,
            rect: Rect {
                pos: Point { x: 0, y: 0 },
                size: self.size,
            },
        };
        f(&mut canvas);
    }
}
