use std::collections::HashMap;

use task_runner::control::control_loop::ControlLoop;
use task_runner::core::core_types::TaskDefinition;
use task_runner::registry::task_registry::{InMemoryTaskRegistry, TaskRegistry};

fn main() {
    let task1 = TaskDefinition {
        message: "hello from task1".to_string(),
        sleep_time_seconds: 4,
        output_path: "task1output".to_string(),
    };
    let task2 = TaskDefinition {
        message: "hello from task2".to_string(),
        sleep_time_seconds: 6,
        output_path: "task2output".to_string(),
    };
    let mut registry = InMemoryTaskRegistry::new(HashMap::new());
    registry.create_task("Task 1", &task1);
    registry.create_task("Task 2", &task2);
    let mut control_loop = ControlLoop::new(&mut registry);

    let mut count = 0u32;
    loop {
        count += 1;
        println!(
            "----------\nRunning control loop count {}\n----------",
            count
        );
        control_loop.run_once();
        std::thread::sleep(std::time::Duration::from_secs(2));

        if count == 10 {
            break;
        }
    }
    println!("All done!");
}
