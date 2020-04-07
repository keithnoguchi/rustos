//! [Simple] dummy waker based executor
//!
//! [simple]: https://os.phil-opp.com/async-await/#simple-executor
use super::{alloc::collections::VecDeque, Task};
use core::{
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

/// Simple dummy waker based executor.
pub struct Executor {
    task_queue: VecDeque<Task>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }
}

impl Executor {
    /// Create new executor.
    pub fn new() -> Self {
        Self::default()
    }
    /// Spawn a new task.
    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }
    /// Run the executor.
    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Pending => self.task_queue.push_back(task),
                Poll::Ready(()) => {}
            }
        }
    }
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(ptr::null(), vtable)
}
