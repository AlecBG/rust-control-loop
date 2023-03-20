use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crate::core::core_types::{TaskState, TaskStatus};
use crate::registry::task_registry::TaskRegistry;
use crate::threadpool::threadpool::ThreadPool;

pub struct ControlLoop<'a> {
    registry: &'a mut dyn TaskRegistry,
    threadpool: ThreadPool,
}

impl ControlLoop<'_> {
    pub fn new(registry: &mut dyn TaskRegistry) -> ControlLoop {
        return ControlLoop {
            registry: registry,
            threadpool: ThreadPool::new(2),
        };
    }

    pub fn run_once(&self) {
        let mut non_terminal_statuses: HashSet<TaskStatus> = HashSet::new();
        non_terminal_statuses.insert(TaskStatus::PENDING);
        non_terminal_statuses.insert(TaskStatus::RUNNING);
        for task in self.registry.get_tasks(&non_terminal_statuses) {
            if task.status == TaskStatus::PENDING {
                self.advance_pending(&task)
            } else if task.status == TaskStatus::RUNNING {
                self.advance_running(&task)
            } else {
                panic!("Unexected status {}", task.status.to_string())
            }
        }
    }

    fn advance_pending(&self, task: &TaskState) {
        let sleep_time = task.sleep_time_seconds;
        let message = (&task.message).to_string();
        self.threadpool.execute(move || {
            thread::sleep(Duration::from_secs(sleep_time as u64));
            println!("{}", message);
        });
    }

    fn advance_running(&self, task: &TaskState) {}
}
