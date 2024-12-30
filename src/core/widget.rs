//! This file contains the implementation of the `Widget` struct.

use std::ops::{Deref, DerefMut};

use super::Ele;

/// This struct is the wrapper of GUI element.
/// It contains the pointer to the underlying element and the address of the widget.
pub struct Widget<T: Ele> {
    _data: Box<(Box<dyn Ele>, bool)>,
    type_data: Option<Box<T>>,
    addr: usize,
}

impl<T: Ele> Deref for Widget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.type_data.as_ref().unwrap().deref()
    }
}

impl<T: Ele> DerefMut for Widget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.type_data.as_mut().unwrap().deref_mut()
    }
}

impl<T: Ele> Widget<T> {
    pub fn new(data: Box<T>) -> Self {
        // make the same address for the type data and the dynamic data
        let type_ptr = Box::into_raw(data);
        let dyn_ptr = type_ptr as *mut dyn Ele;
        let box_ptr = unsafe { Box::from_raw(dyn_ptr) };
        let addr = Box::into_raw(Box::new((box_ptr, false))) as usize;
        Self {
            _data: unsafe { Box::from_raw(addr as *mut (Box<dyn Ele>, bool)) },
            type_data: unsafe { Some(Box::from_raw(type_ptr)) },
            addr,
        }
    }

    pub fn addr(&self) -> usize {
        self.addr
    }
}

impl<T: Ele> Drop for Widget<T> {
    fn drop(&mut self) {
        // drop the type data
        let ty_box = self.type_data.take().unwrap();
        let _ = Box::into_raw(ty_box);
    }
}
