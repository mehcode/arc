use crate::{os, NodeId};
use slotmap::SlotMap;
use std::cell::RefCell;

thread_local! {
    crate static CONTEXT: Context = Context::default();
}

#[derive(Default)]
crate struct Context {
    crate inner: os::Context,
    crate nodes: RefCell<SlotMap<Box<dyn os::Node>>>,
}

// TODO: Is there a better spot for these methods (?)

#[inline]
crate fn emplace<T>(node: T) -> NodeId
where
    T: os::Node + 'static,
{
    // `.emplace` must be used on the main thread
    debug_assert!(os::is_main_thread());

    NodeId(CONTEXT.with(|cx| cx.nodes.borrow_mut().insert(Box::new(node))))
}

#[inline]
crate fn ensure_exists() {
    // `.ensure_exists` must be used on the main thread
    debug_assert!(os::is_main_thread());

    // `.with` will create the context if not yet created
    CONTEXT.with(|_| ());
}

#[inline]
pub fn run() {
    // `.run` must be used on the main thread
    debug_assert!(os::is_main_thread());

    CONTEXT.with(|cx| cx.inner.run());
}
