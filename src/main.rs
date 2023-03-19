use std::collections::HashMap;

use task_runner::control::control_loop::ControlLoop;
use task_runner::core::core_types::{TaskDefinition, TaskState, TaskStatus};
use task_runner::registry::task_registry::{InMemoryTaskRegistry, TaskRegistry};

fn main() {
    let task1 = TaskDefinition {
        message: "hello from task1".to_string(),
        sleep_time_seconds: 4,
    };
    let task2 = TaskDefinition {
        message: "hello from task2".to_string(),
        sleep_time_seconds: 6,
    };
    let mut registry = InMemoryTaskRegistry::new(HashMap::new());
    registry.create_task("Task 1", &task1);
    registry.create_task("Task 2", &task2);
    let control_loop = ControlLoop::new(&mut registry);
    control_loop.run_once()
}
