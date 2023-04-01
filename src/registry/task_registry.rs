use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{self, Display};

use crate::core::core_types::{NewTaskInfo, TaskState, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskNotFoundError {
    pub task_id: String,
}

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no task for task id {}", self.task_id)
    }
}

pub trait TaskRegistry {
    fn get_task(self: &Self, task_id: &str) -> Result<TaskState, TaskNotFoundError>;
    fn update_task_from_control_loop(&mut self, task_id: &str, status: TaskStatus);
    fn create_task(self: &mut Self, new_task_info: &NewTaskInfo) -> TaskState;
    fn get_tasks<'a>(
        self: &'a Self,
        statuses: &'a HashSet<TaskStatus>,
    ) -> Box<dyn Iterator<Item = TaskState> + 'a>;
}

pub struct InMemoryTaskRegistry {
    tasks: HashMap<String, TaskState>,
}

impl InMemoryTaskRegistry {
    pub fn new(tasks: HashMap<String, TaskState>) -> InMemoryTaskRegistry {
        InMemoryTaskRegistry { tasks }
    }
}

impl TaskRegistry for InMemoryTaskRegistry {
    fn get_task(self: &Self, task_id: &str) -> Result<TaskState, TaskNotFoundError> {
        let optional_task = self.tasks.get(task_id);
        if let Some(task) = optional_task {
            Ok(task.clone())
        } else {
            Err(TaskNotFoundError {
                task_id: task_id.to_string(),
            })
        }
    }

    fn update_task_from_control_loop(&mut self, task_id: &str, status: TaskStatus) {
        let task_state = self.get_task(task_id).unwrap();
        let mut new_task_state = task_state.clone();
        new_task_state.status = status.clone();
        self.tasks
            .insert((&new_task_state.name).to_string(), new_task_state);
    }

    fn create_task(self: &mut Self, new_task_info: &NewTaskInfo) -> TaskState {
        let task_state = TaskState::new(new_task_info);
        self.tasks.insert(
            new_task_info.task_id.to_string(),
            TaskState::new(new_task_info),
        );
        task_state
    }

    fn get_tasks<'a>(
        self: &'a Self,
        statuses: &'a HashSet<TaskStatus>,
    ) -> Box<dyn Iterator<Item = TaskState> + 'a> {
        Box::new(
            self.tasks
                .values()
                .filter(|state| statuses.contains(&state.status))
                .map(|task_state| task_state.clone()),
        )
    }
}

impl Display for InMemoryTaskRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for (idx, (key, value)) in self.tasks.iter().enumerate() {
            result.push_str(key);
            result.push_str(": ");
            result.push_str(&value.to_string());
            if idx != self.tasks.len() - 1 {
                result.push_str(", ");
            }
        }
        write!(f, "{}", result)
    }
}
