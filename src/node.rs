use crate::{context::CONTEXT, os};
use slotmap::Key;

pub trait Node {
    fn id(&self) -> NodeId;
}

#[derive(Copy, Clone)]
pub struct NodeId(crate Key);

impl NodeId {
    #[inline(always)]
    crate fn with<T, R, F>(self, callback: F) -> R
    where
        T: os::Node,
        R: Send,
        F: FnOnce(&T) -> R + Send,
    {
        self.with_any(move |node| {
            match node.downcast_ref() {
                Ok(node) => callback(node),

                // This should be impossible to reach as the public API is typesafe.
                Err(_) => unreachable!(),
            }
        })
    }

    #[inline(always)]
    crate fn with_mut<T, R, F>(self, callback: F) -> R
    where
        T: os::Node,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        self.with_any_mut(move |node| {
            match node.downcast_mut() {
                Ok(node) => callback(node),

                // This should be impossible to reach as the public API is typesafe.
                Err(_) => unreachable!(),
            }
        })
    }

    #[inline(always)]
    crate fn with_any<R, F>(self, callback: F) -> R
    where
        R: Send,
        F: FnOnce(&Box<dyn os::Node>) -> R + Send,
    {
        os::execute_on_main_thread(move || CONTEXT.with(|cx| callback(&cx.nodes.borrow()[self.0])))
    }

    #[inline(always)]
    crate fn with_any_mut<R, F>(self, callback: F) -> R
    where
        R: Send,
        F: FnOnce(&mut Box<dyn os::Node>) -> R + Send,
    {
        os::execute_on_main_thread(move || {
            CONTEXT.with(|cx| callback(&mut cx.nodes.borrow_mut()[self.0]))
        })
    }
}
