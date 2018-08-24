use crate::{os, Node, NodeId};
use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
use slotmap::{Key, SlotMap};
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

thread_local! {
    static CONTEXT: Context = Context::default();
}

#[derive(Default)]
crate struct Context {
    inner: os::Context,
    nodes: RefCell<SlotMap<Box<dyn os::Node>>>,
}

impl Context {
    crate fn emplace<T>(node: T) -> Key
    where
        T: os::Node + Send + 'static,
    {
        // `.emplace` must be used on the main thread
        debug_assert!(os::is_main_thread());

        CONTEXT.with(|cx| cx.nodes.borrow_mut().insert(Box::new(node)))
    }

    crate fn with<R, F>(key: Key, callback: F) -> R
    where
        R: Send,
        F: FnOnce(&Box<dyn os::Node>) -> R + Send,
    {
        os::execute_on_main_thread(move || CONTEXT.with(|cx| callback(&cx.nodes.borrow()[key])))
    }

    crate fn with_mut<R, F>(key: Key, callback: F) -> R
    where
        R: Send,
        F: FnOnce(&mut Box<dyn os::Node>) -> R + Send,
    {
        os::execute_on_main_thread(move || {
            CONTEXT.with(|cx| callback(&mut cx.nodes.borrow_mut()[key]))
        })
    }
}
