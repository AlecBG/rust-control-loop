use std::collections::HashSet;
use std::fmt::{self};

use crate::core::core_types::{NewTaskInfo, TaskState, TaskStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct TaskNotFoundError {
    pub task_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskAlreadyExistsError {
    pub task_id: String,
}

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no task for task id {}", self.task_id)
    }
}

pub trait TaskRegistry {
    fn get_task(self: &Self, task_id: &str) -> Result<TaskState, TaskNotFoundError>;
    fn update_task_from_control_loop(&self, task_id: &str, status: TaskStatus);
    fn create_task(
        self: &Self,
        new_task_info: &NewTaskInfo,
    ) -> Result<TaskState, TaskAlreadyExistsError>;
    fn get_tasks<'a>(
        self: &'a Self,
        statuses: &'a HashSet<TaskStatus>,
    ) -> Box<dyn Iterator<Item = TaskState> + 'a>;
}
