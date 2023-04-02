use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub struct NewTaskInfo {
    pub task_id: String,
    pub task_definition: TaskDefinition,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    PENDING,
    RUNNING,
    FAILED,
    SUCCESS,
}

impl std::str::FromStr for TaskStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TaskStatus, Self::Err> {
        match input {
            "PENDING" => Ok(TaskStatus::PENDING),
            "RUNNING" => Ok(TaskStatus::RUNNING),
            "FAILED" => Ok(TaskStatus::FAILED),
            "SUCCESS" => Ok(TaskStatus::SUCCESS),
            _ => Err(()),
        }
    }
}

impl std::string::ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::PENDING => "PENDING".to_string(),
            TaskStatus::RUNNING => "RUNNING".to_string(),
            TaskStatus::FAILED => "FAILED".to_string(),
            TaskStatus::SUCCESS => "SUCCESS".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskDefinition {
    pub sleep_time_seconds: u16,
    pub message: String,
    pub output_path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaskState {
    pub status: TaskStatus,
    pub name: String,
    pub sleep_time_seconds: u16,
    pub message: String,
    pub output_path: String,
}

impl Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Task(name={}, status={})",
            self.name,
            self.status.to_string()
        )
    }
}

impl TaskState {
    pub fn new(new_task_info: &NewTaskInfo) -> TaskState {
        TaskState {
            status: TaskStatus::PENDING,
            name: new_task_info.task_id.to_string(),
            sleep_time_seconds: new_task_info.task_definition.sleep_time_seconds,
            message: new_task_info.task_definition.message.to_string(),
            output_path: new_task_info.task_definition.output_path.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        thread::sleep(Duration::from_secs(self.sleep_time_seconds as u64));
        println!("{}", &self.message);
        // Write message to output_path
        let mut file = File::create(&self.output_path)?;
        file.write_all(self.message.as_bytes())?;
        Ok(())
    }
}
