use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
            _ => panic!("unknown status"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskDefinition {
    pub sleep_time_seconds: u16,
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaskState {
    pub status: TaskStatus,
    pub name: String,
    pub sleep_time_seconds: u16,
    pub message: String,
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
    pub fn new(task_name: &str, task_definition: &TaskDefinition) -> TaskState {
        TaskState {
            status: TaskStatus::PENDING,
            name: task_name.to_string(),
            sleep_time_seconds: task_definition.sleep_time_seconds,
            message: task_definition.message.to_string(),
        }
    }
}
