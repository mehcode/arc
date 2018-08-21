use crate::{os, Node, NodeId};
use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Weak,
};

pub struct Context {
    inner: os::Context,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: os::Context::default(),
        }
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}
