//! Wake-abling async task executor
use super::{
    alloc::{
        collections::{BTreeMap, VecDeque},
        sync::Arc,
        task::Wake,
    },
    Task, TaskId,
};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

const WAKE_QUEUE_SIZE: usize = 100;

pub struct Executor {
    run_queue: VecDeque<Task>,
    wait_queue: BTreeMap<TaskId, Task>,
    wake_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            run_queue: VecDeque::new(),
            wait_queue: BTreeMap::new(),
            wake_queue: Arc::new(ArrayQueue::new(WAKE_QUEUE_SIZE)),
            waker_cache: BTreeMap::new(),
        }
    }
}

impl Executor {
    /// Create new executor.
    pub fn new() -> Self {
        let mut executor = Self::default();
        executor.spawn(Task::new(super::keyboard::print_keypress()));
        executor
    }
    /// Spawn a new task.
    pub fn spawn(&mut self, task: Task) {
        self.run_queue.push_back(task)
    }
    /// Run executor.
    pub fn run(&mut self) -> ! {
        loop {
            self.wake_tasks();
            self.poll_tasks();
            self.sleep_if_idle();
        }
    }
    /// Wake up tasks
    fn wake_tasks(&mut self) {
        while let Ok(task_id) = self.wake_queue.pop() {
            if let Some(task) = self.wait_queue.remove(&task_id) {
                self.run_queue.push_back(task);
            }
        }
    }
    /// Poll ready tasks.
    fn poll_tasks(&mut self) {
        while let Some(mut task) = self.run_queue.pop_front() {
            let task_id = task.id;
            #[allow(clippy::map_entry)]
            if !self.waker_cache.contains_key(&task_id) {
                self.waker_cache.insert(task_id, self.create_waker(task_id));
            }
            let waker = self.waker_cache.get(&task_id).expect("should exit");
            let mut ctx = Context::from_waker(waker);
            match task.poll(&mut ctx) {
                Poll::Ready(()) => {
                    self.waker_cache.remove(&task_id);
                }
                Poll::Pending => {
                    if self.wait_queue.insert(task_id, task).is_some() {
                        panic!("task with same ID already in wait_queue");
                    }
                }
            }
        }
    }
    fn sleep_if_idle(&self) {
        // first path.
        if !self.wake_queue.is_empty() {
            return;
        }
        // Disable interrupt before checking the wake queue,
        // otherwise the interrupt handler might be able to
        // add task after the wake queue check happens below.
        x86_64::instructions::interrupts::disable();
        if self.wake_queue.is_empty() {
            // sleep until the next interrupt.
            x86_64::instructions::interrupts::enable_interrupts_and_hlt();
        } else {
            x86_64::instructions::interrupts::enable();
        }
    }
    fn create_waker(&self, task_id: TaskId) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            id: task_id,
            wake_queue: Arc::clone(&self.wake_queue),
        }))
    }
}

struct TaskWaker {
    id: TaskId,
    wake_queue: Arc<ArrayQueue<TaskId>>,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

impl TaskWaker {
    fn wake_task(&self) {
        self.wake_queue.push(self.id).expect("wake_queue is full");
    }
}
