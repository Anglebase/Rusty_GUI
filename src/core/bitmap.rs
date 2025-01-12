use crate::*;
use std::os::raw::c_void;

pub struct BitMap {
    pub(crate) bmp: *mut c_void,
    pub(crate) size: Size,
}

impl Drop for BitMap {
    fn drop(&mut self) {
        delete_object(self.bmp);
    }
}

impl BitMap {
    pub fn new_with_canvas(canvas: &Canvas, size: Size) -> Self {
        BitMap {
            bmp: new_bitmap(canvas.hdc, size.width, size.height),
            size,
        }
    }

    pub fn new_from_canvas(canvas: &Canvas) -> Self {
        let size = canvas.rect.size;
        BitMap {
            bmp: new_bitmap(canvas.hdc, size.width, size.height),
            size,
        }
    }

    pub fn canvas<F: FnMut(&mut Canvas)>(&self, canvas: &Canvas, mut f: F) {
        let mdc = new_hdc(canvas.hdc);
        select_object(mdc, self.bmp as _);
        let mut canvas: Canvas = Canvas {
            hdc: mdc,
            rect: Rect {
                pos: Point { x: 0, y: 0 },
                size: self.size,
            },
        };
        f(&mut canvas);
        delete_hdc(mdc);
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
