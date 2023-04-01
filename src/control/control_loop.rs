use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crate::core::core_types::{TaskState, TaskStatus};
use crate::registry::task_registry::TaskRegistry;
use crate::threadpool::threadpool::ThreadPool;

struct TaskInfo {
    task_id: String,
    status: TaskStatus,
}

pub struct ControlLoop<'a> {
    registry: &'a mut dyn TaskRegistry,
    threadpool: ThreadPool,
    sender: Sender<TaskInfo>,
    receiver: Receiver<TaskInfo>,
}

impl ControlLoop<'_> {
    pub fn new(registry: &mut dyn TaskRegistry) -> ControlLoop {
        let (sender, receiver) = mpsc::channel::<TaskInfo>();
        return ControlLoop {
            registry: registry,
            threadpool: ThreadPool::new(2),
            sender,
            receiver,
        };
    }

    pub fn run_once(&mut self) {
        for task in self
            .registry
            .get_tasks(&HashSet::from([TaskStatus::PENDING]))
        {
            self.trigger_pending(&task)
        }
        self.advance();
    }

    fn trigger_pending(&self, task: &TaskState) {
        assert_eq!(task.status, TaskStatus::PENDING);
        let sender = (&self.sender).clone();
        let cloned_task = task.clone();
        self.threadpool.execute(move || {
            sender
                .send(TaskInfo {
                    task_id: cloned_task.name.to_string(),
                    status: TaskStatus::RUNNING,
                })
                .unwrap();
            let result = cloned_task.run();
            match result {
                Ok(_) => {
                    println!("Task {} succeeded", cloned_task.name);
                    sender
                        .send(TaskInfo {
                            task_id: cloned_task.name.to_string(),
                            status: TaskStatus::SUCCESS,
                        })
                        .unwrap();
                }
                Err(_) => {
                    println!("Task {} failed", cloned_task.name);
                    sender
                        .send(TaskInfo {
                            task_id: cloned_task.name.to_string(),
                            status: TaskStatus::FAILED,
                        })
                        .unwrap();
                }
            }
        });
    }

    fn advance(&mut self) {
        let mut iterator = self.receiver.try_iter();
        loop {
            let try_received_task = iterator.next();
            if try_received_task.is_none() {
                break;
            }
            let received_task = try_received_task.unwrap();
            print!(
                "Updating task {} to status {}...",
                received_task.task_id,
                received_task.status.to_string()
            );
            self.registry.update_task_from_control_loop(
                &received_task.task_id,
                (&received_task.status).clone(),
            );
        }
    }
}
