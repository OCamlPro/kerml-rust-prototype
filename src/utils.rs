use std::{cell::RefCell, rc::Rc};

pub fn new_rc_ref<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}
