//! This file contains the implementation of the `Notifier` struct and `Responder` struct.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// A wrapper of callback function.
/// The generic parameter `T` is the type of data passed to the callback function.
/// It will be passed as an immutable reference to the callback function.
pub struct Responder<T> {
    f: Rc<RefCell<dyn FnMut(&T)>>,
}

impl<T> Deref for Responder<T> {
    type Target = Rc<RefCell<dyn FnMut(&T)>>;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

impl<T> DerefMut for Responder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.f
    }
}

impl<T> Responder<T> {
    /// Create a new `Responder` instance with the given function.
    pub fn new<F: FnMut(&T) + 'static>(f: F) -> Self {
        Self { f: Rc::new(RefCell::new(f)) }
    }
}

/// A struct to manage callback functions.
/// The generic parameter `T` is the type of data passed to the callback functions.
/// It will be passed as an immutable reference to the callback functions.
/// If you want to pass multiple data to the callback functions, you can wrap them in a tuple.
pub struct Notifier<T> {
    responders: HashMap<String, Responder<T>>,
    disable: HashSet<String>,
}

impl<T> Notifier<T> {
    /// Create a new `Notifier` instance.
    pub fn new() -> Self {
        Self {
            responders: HashMap::new(),
            disable: HashSet::new(),
        }
    }

    /// Check if the `Notifier` has a callback function with the given name.
    pub fn has(&self, name: &str) -> bool {
        self.responders.contains_key(name)
    }

    /// Check if the `Notifier` has a disabled callback function with the given name.
    pub fn disabled(&self, name: &str) -> bool {
        self.disable.contains(name)
    }

    /// Disable the callback function with the given name.
    pub fn disable(&mut self, name: &str) {
        if self.has(name) {
            self.disable.insert(name.to_string());
        }
    }

    /// Enable the callback function with the given name.
    pub fn enable(&mut self, name: &str) {
        self.disable.remove(name);
    }

    /// Add a new callback function with the given name and function.
    pub fn add(&mut self, name: &str, f: Responder<T>) {
        self.responders.insert(name.to_string(), f);
    }

    /// Remove the callback function with the given name.
    pub fn remove(&mut self, name: &str) {
        self.responders.remove(name);
    }

    /// Call all callback functions with the given data.
    pub fn notify(&mut self, data: &T) {
        for (name, f) in self.responders.iter_mut() {
            if !self.disable.contains(name) {
                let mut f = RefCell::borrow_mut(f);
                f(data);
            }
        }
    }
}
