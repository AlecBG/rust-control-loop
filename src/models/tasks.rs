use crate::core::core_types::{TaskDefinition, TaskState, TaskStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDefinitionModel {
    pub sleep_time_seconds: u16,
    pub message: String,
    pub output_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStateModel {
    pub status: TaskStatus,
    pub name: String,
    pub sleep_time_seconds: u16,
    pub message: String,
    pub output_path: String,
}

impl TaskStateModel {
    pub fn from_task_state(task_state: &TaskState) -> TaskStateModel {
        TaskStateModel {
            status: (&task_state.status).clone(),
            name: task_state.name.to_string(),
            sleep_time_seconds: task_state.sleep_time_seconds,
            message: task_state.message.to_string(),
            output_path: task_state.output_path.to_string(),
        }
    }
}

impl TaskDefinitionModel {
    pub fn create_task_definition(&self) -> TaskDefinition {
        TaskDefinition {
            sleep_time_seconds: self.sleep_time_seconds,
            message: self.message.to_string(),
            output_path: self.output_path.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskDefinitionResponse {
    pub task_id: String,
    pub task_definition: TaskDefinitionModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTaskStateResponse {
    pub task_id: String,
    pub task_state: TaskStateModel,
}
