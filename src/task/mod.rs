//! Task manager
extern crate alloc;
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

mod executor;
pub(crate) mod keyboard;
mod simple;

/// Re-exports.
pub use executor::Executor;

/// Async task.
pub struct Task {
    #[allow(dead_code)]
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new task.
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }
    fn poll(&mut self, ctx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use crate::{serial_print, serial_println};
    #[test_case]
    fn initial_task_id() {
        serial_print!("task::initial_task_id... ");
        let task = super::Task::new(initial_test_task());
        assert_eq!(0, task.id.0);
        serial_println!("[ok]");
    }
    async fn initial_test_task() {
        ()
    }
}
