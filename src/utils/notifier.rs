//! This file contains the implementation of the `Notifier` struct and `Responder` struct.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// `Responder` struct is a wrapper for a callback function.
/// The generic parameter `T` represents the type of data passed to the callback function.
/// The data will be passed as an immutable reference to the callback function.
/// `Rc<RefCell<...>>` is used to allow the callback function to be shared among multiple `Notifier` instances and to be mutable within the struct.
#[derive(Clone)]
pub struct Responder<T> {
    f: Rc<RefCell<dyn FnMut(&T)>>,
}

impl<T> Deref for Responder<T> {
    type Target = Rc<RefCell<dyn FnMut(&T)>>;

    /// Returns an immutable reference to the `Rc<RefCell<FnMut(&T)>>` inside the `Responder`.
    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

impl<T> DerefMut for Responder<T> {
    /// Returns a mutable reference to the `Rc<RefCell<FnMut(&T)>>` inside the `Responder`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.f
    }
}

impl<T> Responder<T> {
    /// Creates a new `Responder` instance with the given function.
    /// 
    /// # Parameters
    /// - `f`: A function that takes an immutable reference to type `T` and returns nothing. This function must implement the `FnMut` trait and have a `'static` lifetime.
    pub fn new<F: FnMut(&T) + 'static>(f: F) -> Self {
        Self { f: Rc::new(RefCell::new(f)) }
    }
}

/// `Notifier` struct is used to manage callback functions.
/// The generic parameter `T` represents the type of data passed to the callback functions.
/// The data will be passed as an immutable reference to the callback functions.
/// If you want to pass multiple data to the callback functions, you can wrap them in a tuple.
/// `Notifier` uses a `HashMap` to store the callback functions and a `HashSet` to track which callbacks are disabled.
pub struct Notifier<T> {
    responders: HashMap<String, Responder<T>>,
    disable: HashSet<String>,
}

impl<T> Notifier<T> {
    /// Creates a new `Notifier` instance with empty `responders` and `disable` collections.
    pub fn new() -> Self {
        Self {
            responders: HashMap::new(),
            disable: HashSet::new(),
        }
    }

    /// Checks if the `Notifier` has a callback function with the given name.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function to check.
    /// 
    /// # Returns
    /// - `bool`: `true` if the callback function exists, `false` otherwise.
    pub fn has(&self, name: &str) -> bool {
        self.responders.contains_key(name)
    }

    /// Checks if the `Notifier` has a disabled callback function with the given name.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function to check.
    /// 
    /// # Returns
    /// - `bool`: `true` if the callback function is disabled, `false` otherwise.
    pub fn disabled(&self, name: &str) -> bool {
        self.disable.contains(name)
    }

    /// Disables the callback function with the given name.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function to disable.
    pub fn disable(&mut self, name: &str) {
        if self.has(name) {
            self.disable.insert(name.to_string());
        }
    }

    /// Enables the callback function with the given name.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function to enable.
    pub fn enable(&mut self, name: &str) {
        self.disable.remove(name);
    }

    /// Adds a new callback function with the given name and function to the `Notifier`.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function.
    /// - `f`: A `Responder` instance containing the callback function.
    pub fn add(&mut self, name: &str, f: Responder<T>) {
        self.responders.insert(name.to_string(), f);
    }

    /// Removes the callback function with the given name from the `Notifier`.
    /// 
    /// # Parameters
    /// - `name`: The name of the callback function to remove.
    pub fn remove(&mut self, name: &str) {
        self.responders.remove(name);
    }

    /// Calls all enabled callback functions with the given data.
    /// 
    /// # Parameters
    /// - `data`: The data to pass to the callback functions, passed as an immutable reference.
    pub fn notify(&mut self, data: &T) {
        for (name, f) in self.responders.iter_mut() {
            if !self.disable.contains(name) {
                let mut f = RefCell::borrow_mut(f);
                f(data);
            }
        }
    }
}
