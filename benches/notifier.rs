

#[cfg(test)]
mod tests {
    use rusty_gui::{Responder, Notifier};
    use std::{cell::RefCell, rc::Rc, sync::atomic::{AtomicBool, Ordering}};

    #[test]
    fn test_notifier_happy_path() {
        let mut notifier = Notifier::new();
        let flag = Rc::new(AtomicBool::new(false));

        let flag_clone = Rc::clone(&flag);
        let responder = Responder::new(move |_: &i32| {
            flag_clone.store(true, Ordering::SeqCst);
        });

        notifier.add("test", responder);
        notifier.notify(&42);

        assert_eq!(flag.load(Ordering::SeqCst), true);
    }

    #[test]
    fn test_notifier_add_and_has() {
        let mut notifier = Notifier::new();
        let responder = Responder::new(|_: &i32| {});

        notifier.add("test", responder);
        assert_eq!(notifier.has("test"), true);
        assert_eq!(notifier.has("nonexistent"), false);
    }

    #[test]
    fn test_notifier_remove() {
        let mut notifier = Notifier::new();
        let responder = Responder::new(|_: &i32| {});

        notifier.add("test", responder);
        assert_eq!(notifier.has("test"), true);

        notifier.remove("test");
        assert_eq!(notifier.has("test"), false);
    }

    #[test]
    fn test_notifier_disable_and_enable() {
        let mut notifier = Notifier::new();
        let responder = Responder::new(|_: &i32| {});

        notifier.add("test", responder);

        notifier.disable("test");
        assert_eq!(notifier.disabled("test"), true);
        assert_eq!(notifier.has("test"), true);

        notifier.enable("test");
        assert_eq!(notifier.disabled("test"), false);
        assert_eq!(notifier.has("test"), true);
    }

    #[test]
    fn test_notifier_notify_disabled_responder() {
        let mut notifier = Notifier::new();
        let flag = Rc::new(AtomicBool::new(false));

        let flag_clone = Rc::clone(&flag);
        let responder = Responder::new(move |_: &i32| {
            flag_clone.store(true, Ordering::SeqCst);
        });

        notifier.add("test", responder);
        notifier.disable("test");
        notifier.notify(&42);

        assert_eq!(flag.load(Ordering::SeqCst), false);
    }

    #[test]
    fn test_notifier_notify_no_responders() {
        let mut notifier = Notifier::<i32>::new();
        notifier.notify(&42);

        // Nothing to assert here, but this should not panic
    }

    #[test]
    fn test_notifier_add_duplicate_responder() {
        let mut notifier = Notifier::new();
        let responder1 = Responder::new(|_: &i32| {});
        let responder2 = Responder::new(|_: &i32| {});

        notifier.add("test", responder1);
        notifier.add("test", responder2);
        assert_eq!(notifier.has("test"), true);
    }

    #[test]
    fn test_common_responder() {
        let mut notifier = Notifier::new();

        let flag = Rc::new(RefCell::new(false));
        let flag_clone = Rc::clone(&flag);
        let responder = Responder::new(
            move |_: &i32| {
                *flag.borrow_mut() = true;
            }
        );
        notifier.add("test", responder);

        assert_eq!(*flag_clone.borrow(), false);
        notifier.notify(&42);
        assert_eq!(*flag_clone.borrow(), true);
    }
}