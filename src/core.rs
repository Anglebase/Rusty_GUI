use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Clone)]
pub enum Responder<Args> {
    Fn(Rc<RefCell<dyn Fn(&Args)>>),
    FnMut(Rc<RefCell<dyn FnMut(&Args)>>),
}

#[derive(Clone)]
pub struct Notifier<Args> {
    notifiers: HashMap<String, Responder<Args>>,
    disable_list: HashSet<String>,
}

impl<Args> Notifier<Args> {
    pub fn new() -> Self {
        Self {
            notifiers: HashMap::new(),
            disable_list: HashSet::new(),
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.notifiers.contains_key(name)
    }

    pub fn disable(&mut self, name: &str) {
        if self.has(name) {
            self.disable_list.insert(name.to_string());
        }
    }

    pub fn enable(&mut self, name: &str) {
        self.disable_list.remove(name);
    }

    pub fn connect(&mut self, name: &str, responder: Rc<RefCell<dyn Fn(&Args)>>) {
        self.notifiers
            .insert(name.to_string(), Responder::Fn(responder));
    }

    pub fn connect_mut(&mut self, name: &str, responder: Rc<RefCell<dyn FnMut(&Args)>>) {
        self.notifiers
            .insert(name.to_string(), Responder::FnMut(responder));
    }

    pub fn disconnect(&mut self, name: &str) {
        self.notifiers.remove(name);
    }

    pub fn notify(&self, args: Args) {
        for (name, responder) in self.notifiers.iter() {
            if self.disable_list.contains(name) {
                continue;
            }
            match responder {
                Responder::Fn(f) => {
                    let f = f.borrow();
                    f(&args);
                }
                Responder::FnMut(f) => {
                    let mut f: RefMut<'_, dyn FnMut(&Args)> = RefCell::borrow_mut(f);
                    f(&args);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! respond {
    (($($arg_name:ident : $arg_type:ty),*) => { $($body:tt)* }) => {
         std::rc::Rc::new(std::cell::RefCell::new(move | ($($arg_name),*):($($arg_type),*) | {
             $($body)*
         }))
    };
}
