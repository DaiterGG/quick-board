use crate::{Action, ActionPump};

pub struct Observed<T: Sized> {
    value: T,
    action: Box<dyn Fn(T) -> Action>,
}
impl<T: Copy> Observed<T> {
    pub fn get(&self) -> T {
        self.value
    }
    pub fn new_silent(value: T, action: Box<dyn Fn(T) -> Action>) -> Observed<T> {
        Observed { value, action }
    }
    // NOTE: by default action is notified on init
    pub fn new(value: T, action: Box<dyn Fn(T) -> Action>) -> Observed<T> {
        ActionPump::add(action(value));
        Observed { value, action }
    }
    pub fn set(&mut self, value: T) {
        self.value = value;
        ActionPump::add((self.action)(self.value));
    }
}
