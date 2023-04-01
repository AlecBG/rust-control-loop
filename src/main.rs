use std::sync::mpsc;

use task_runner::control::control_loop::ControlLoop;
use task_runner::core::core_types::{NewTaskInfo, TaskDefinition};
use task_runner::registry::task_registry_sqlite::{TablePermanance, TaskRegistrySqlite};

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

    let mut registry =
        TaskRegistrySqlite::new(":memory:", "test_table", TablePermanance::DropOnClose);
    let (sender, receiver) = mpsc::channel::<NewTaskInfo>();
    let new_task1 = NewTaskInfo {
        task_id: "Task 1".to_string(),
        task_definition: task1,
    };
    let new_task2 = NewTaskInfo {
        task_id: "Task 2".to_string(),
        task_definition: task2,
    };
    sender.send(new_task1).unwrap();
    sender.send(new_task2).unwrap();
    let mut control_loop = ControlLoop::new(&mut registry, receiver);

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
