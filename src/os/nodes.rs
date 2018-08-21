use super::{execute_on_main_thread, is_main_thread, Node};
use slotmap::{Key, SlotMap};
use std::cell::RefCell;

pub type NodeId = Key;

thread_local! {
    static NODES: RefCell<Nodes> = RefCell::new(Nodes::default());
}

#[derive(Default)]
pub(crate) struct Nodes(SlotMap<Box<dyn Node>>);

impl Nodes {
    pub(crate) fn emplace(node: impl Node + 'static) -> NodeId {
        // `.emplace` must be used on the main thread
        debug_assert!(is_main_thread());

        NODES.with(|nodes| nodes.borrow_mut().0.insert(Box::new(node)))
    }

    pub(crate) fn with<T: Node, R: Send>(key: Key, callback: impl FnOnce(&mut T) -> R + Send) -> R {
        execute_on_main_thread(move || {
            NODES.with(|nodes| {
                // FIXME: Better error message there
                let nodes = &mut nodes.borrow_mut().0;
                let node: &mut T = nodes[key].downcast_mut().expect("mismatched types");
                callback(node)
            })
        })
    }

    pub(crate) fn with_untyped<R: Send>(
        key: Key,
        callback: impl FnOnce(&mut Box<dyn Node>) -> R + Send,
    ) -> R {
        execute_on_main_thread(move || {
            NODES.with(|nodes| {
                let nodes = &mut nodes.borrow_mut().0;
                callback(&mut nodes[key])
            })
        })
    }

    pub(crate) fn with2_untyped<R: Send>(
        keys: (Key, Key),
        callback: impl FnOnce(&Box<dyn Node>, &Box<dyn Node>) -> R + Send,
    ) -> R {
        execute_on_main_thread(move || {
            NODES.with(|nodes| {
                let nodes = &nodes.borrow().0;
                callback(&nodes[keys.0], &nodes[keys.1])
            })
        })
    }
}
