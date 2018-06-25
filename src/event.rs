pub struct Event<T> {
    handlers: Vec<Box<Fn(&T) -> () + Send + 'static>>,
}

// TODO: Return a handle to the event handler that can be acted on to unsubscribe (?)

impl<T> Event<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: impl Fn(&T) -> () + Send + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub(crate) fn dispatch(&self, event: T) {
        for handler in &self.handlers {
            handler(&event);
        }
    }
}
