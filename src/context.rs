use crate::{os, Node, NodeId};
use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Weak,
};

#[derive(Clone, Default)]
pub struct Context {
    inner: Arc<InnerContext>,
}

#[derive(Default)]
struct InnerContext {
    inner: os::Context,
    lock: Mutex<()>,
    nodes: RwLock<FnvHashMap<NodeId, Box<dyn Node>>>,
    next_node_id: AtomicUsize,
}

unsafe impl Send for InnerContext {}
unsafe impl Sync for InnerContext {}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn downgrade(&self) -> WeakContext {
        WeakContext {
            inner: Arc::downgrade(&self.inner),
        }
    }

    pub fn run(&self) {
        let _guard = self.inner.lock.lock();
        self.inner.inner.run();
    }

    pub fn update_by_id<T: Node>(&self, id: NodeId, callback: impl FnOnce(&mut T) -> () + Send) {
        if let Some(node) = self.inner.nodes.write().get_mut(&id) {
            self.inner.inner.execute_on_main_thread(move || {
                callback(node.downcast_mut().expect("mismatched types in callback"));
            });
        }
    }

    pub(crate) fn next_id(&self) -> NodeId {
        NodeId(self.inner.next_node_id.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn emplace_node(&self, node: impl Node + 'static) {
        self.inner.nodes.write().insert(node.id(), Box::new(node));
    }
}

#[derive(Clone)]
pub struct WeakContext {
    inner: Weak<InnerContext>,
}

impl WeakContext {
    pub fn upgrade(&self) -> Option<Context> {
        Some(Context {
            inner: self.inner.upgrade()?,
        })
    }
}
