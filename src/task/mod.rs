//! Task manager
extern crate alloc;
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

mod simple;

/// Re-exports.
pub use simple::Executor;

/// Async task.
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new task.
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future: Box::pin(future),
        }
    }
    fn poll(&mut self, ctx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(ctx)
    }
}
