//! This file contains the implementation of the `Widget` struct.

use std::ops::{Deref, DerefMut};

use crate::{send_window_created_msg, Rect};

use super::{AbstractElement, Element, Window};

/// This struct is the wrapper of GUI element.
/// It contains the pointer to the underlying element and the address of the widget.
pub struct Widget<T: Element> {
    _data: Box<(Box<dyn AbstractElement>, bool)>,
    type_data: Option<Box<T>>,
    addr: usize,
}

impl<T: Element> Deref for Widget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.type_data.as_ref().unwrap().deref()
    }
}

impl<T: Element> DerefMut for Widget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.type_data.as_mut().unwrap().deref_mut()
    }
}

impl<T: Element> Widget<T> {
    /// Create a new `Widget` with the given data.
    /// The parameter `title`, `rect`, `parent` will be used to call `Window::new` to create the window.
    pub fn new(title: &str, rect: Rect, parent: Option<&Window>) -> Self {
        let data = Box::new(T::default());
        // make the same address for the type data and the dynamic data
        let type_ptr = Box::into_raw(data);
        let dyn_ptr = type_ptr as *mut dyn AbstractElement;
        let box_ptr = unsafe { Box::from_raw(dyn_ptr) };
        let addr = Box::into_raw(Box::new((box_ptr, false))) as usize;
        let mut ret = Self {
            _data: unsafe { Box::from_raw(addr as *mut (Box<dyn AbstractElement>, bool)) },
            type_data: unsafe { Some(Box::from_raw(type_ptr)) },
            addr,
        };
        *ret.as_window_mut() = Window::new(title, rect, parent, &ret);
        send_window_created_msg(ret.as_window().id.hwnd);
        ret
    }

    pub(crate) fn addr(&self) -> usize {
        self.addr
    }
}

impl<T: Element> Drop for Widget<T> {
    fn drop(&mut self) {
        // drop the type data
        let ty_box = self.type_data.take().unwrap();
        let _ = Box::into_raw(ty_box);
    }
}
