use parking_lot::Mutex;
use std::sync::Arc;

pub struct Event<T: Send> {
    handlers: Arc<Mutex<Vec<Box<dyn FnMut(&T) -> () + Send + 'static>>>>,
}

// TODO: Return a handle to the event handler that can be acted on to unsubscribe (?)

impl<T: Send> Event<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, handler: impl FnMut(&T) -> () + Send + 'static) {
        self.handlers.lock().push(Box::new(handler));
    }

    pub(crate) fn dispatch(&mut self, event: &T) {
        for handler in self.handlers.lock().iter_mut() {
            handler(&event);
        }
    }
}

impl<T: Send> Default for Event<T> {
    fn default() -> Self {
        Self {
            handlers: Default::default(),
        }
    }
}

impl<T: Send> Clone for Event<T> {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
        }
    }
}
