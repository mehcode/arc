use super::os;
use super::Window;

pub struct Application {
    inner: os::Application,
}

impl Application {
    pub fn new() -> Self {
        Self {
            inner: os::Application::new(),
        }
    }

    pub fn add_window(&self, window: Window) {
        self.inner.add_window(window.inner);
    }

    pub fn run(&self) {
        self.inner.run();
    }
}
