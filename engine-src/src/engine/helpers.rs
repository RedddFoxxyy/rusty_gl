use std::rc::Rc;

use crate::{TContext, T_CONTEXT};

/// Returns a reference(Reference Counting wrapped reference) to the
/// Global Context object T_CONTEXT.
pub fn get_tctx() -> Rc<TContext> {
    T_CONTEXT.with(|context| {
        context
            .get()
            .expect("Global Context not been initialised")
            .clone()
    })
}
