use super::{os, View, Window};
use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Weak,
};

#[derive(Clone)]
pub struct Context {
    inner: Arc<InnerContext>,
}

struct InnerContext {
    inner: os::Context,
    lock: Mutex<()>,
    nodes: RwLock<FnvHashMap<usize, View>>,
    next_node_id: AtomicUsize,
}

unsafe impl Send for InnerContext {}
unsafe impl Sync for InnerContext {}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InnerContext {
                lock: Mutex::new(()),
                nodes: RwLock::new(FnvHashMap::default()),
                next_node_id: AtomicUsize::new(0),
                inner: os::Context::new(),
            }),
        }
    }

    pub(crate) fn downgrade(&self) -> WeakContext {
        WeakContext {
            inner: Arc::downgrade(&self.inner),
        }
    }

    pub fn add_window(&self, window: Window) {
        let _guard = self.inner.lock.lock();
        self.inner.inner.add_window(window.inner);
    }

    pub fn run(&self) {
        let _guard = self.inner.lock.lock();
        self.inner.inner.run();
    }

    pub fn update_by_id(&self, id: usize, callback: impl FnOnce(&mut View) -> () + Send) {
        if let Some(node) = self.inner.nodes.write().get_mut(&id) {
            self.inner.inner.execute_on_main_thread(move || {
                callback(node);
            });
        }
    }

    pub(crate) fn next_id(&self) -> usize {
        self.inner.next_node_id.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn emplace_node(&self, node: View) -> usize {
        let id = node.id;
        self.inner.nodes.write().insert(id, node);
        id
    }
}

#[derive(Clone)]
pub(crate) struct WeakContext {
    inner: Weak<InnerContext>,
}

impl WeakContext {
    pub(crate) fn upgrade(&self) -> Option<Context> {
        Some(Context {
            inner: self.inner.upgrade()?,
        })
    }
}
