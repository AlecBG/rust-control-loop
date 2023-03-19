use std::collections::HashSet;
use std::str::FromStr;

use crate::core::core_types::{TaskDefinition, TaskState, TaskStatus};
use crate::registry::task_registry;

use sqlite;

const TABLE_NAME: &str = "task_registry";

type SerialisedTaskState = (String, String, i64, String);

fn serialise_task_state(task_state: &TaskState) -> SerialisedTaskState {
    (
        task_state.status.to_string(),
        task_state.name.to_string(),
        task_state.sleep_time_seconds as i64,
        task_state.message.to_string(),
    )
}

fn deserialise_task_state(serialise_task_state: SerialisedTaskState) -> TaskState {
    TaskState {
        status: TaskStatus::from_str(&serialise_task_state.0).unwrap(),
        name: serialise_task_state.1,
        sleep_time_seconds: serialise_task_state.2 as u16,
        message: serialise_task_state.3,
    }
}

pub struct TaskRegistrySqlite {
    connection: sqlite::Connection,
}

impl TaskRegistrySqlite {
    pub fn new() -> TaskRegistrySqlite {
        let query = format!("CREATE TABLE {TABLE_NAME} (status TEXT, name TEXT, sleep_time_seconds INTEGER, message TEXT);");
        let connection = sqlite::Connection::open(":memory:").unwrap();
        connection.execute(query).unwrap();
        TaskRegistrySqlite { connection }
    }
}

impl task_registry::TaskRegistry for TaskRegistrySqlite {
    fn get_task(
        self: &Self,
        task_id: &str,
    ) -> Result<TaskState, task_registry::TaskNotFoundError> {
        let query = format!("SELECT * FROM {TABLE_NAME} WHERE name = ?");
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, task_id)).unwrap();
        let mut cursor = statement.iter();
        let optional_values = cursor.try_next().unwrap();
        if let Some(values) = optional_values {
            let serialised_task_state: SerialisedTaskState = (
                match &values[0] {
                    sqlite::Value::String(i) => i.to_string(),
                    _ => panic!(),
                },
                match &values[1] {
                    sqlite::Value::String(i) => i.to_string(),
                    _ => panic!(),
                },
                match &values[2] {
                    sqlite::Value::Integer(i) => *i,
                    _ => panic!(),
                },
                match &values[3] {
                    sqlite::Value::String(i) => i.to_string(),
                    _ => panic!(),
                },
            );
            Ok(deserialise_task_state(serialised_task_state))
        } else {
            Err(task_registry::TaskNotFoundError {
                task_id: task_id.to_string(),
            })
        }
    }

    fn update_task_from_control_loop(&mut self, task_id: &str, status: TaskStatus) {
        todo!();
    }

    fn create_task(self: &mut Self, task_id: &str, task_definition: &TaskDefinition) -> TaskState {
        let task_state = TaskState::new(task_id, task_definition);
        let serialised_state = serialise_task_state(&task_state);
        let query = format!(
            "INSERT INTO {TABLE_NAME}  VALUES (:status, :name, :sleep_time_seconds, :message) "
        );
        let mut statement = self.connection.prepare(query).unwrap();
        statement
            .bind_iter::<_, (_, sqlite::Value)>([
                (":status", serialised_state.0.into()),
                (":name", serialised_state.1.into()),
                (":sleep_time_seconds", serialised_state.2.into()),
                (":message", serialised_state.3.into()),
            ])
            .unwrap();
        let state = statement.next().unwrap();
        assert_eq!(state, sqlite::State::Done);
        task_state
    }

    fn get_tasks<'a>(
        self: &'a Self,
        statuses: &'a HashSet<TaskStatus>,
    ) -> Box<dyn Iterator<Item = &TaskState> + 'a> {
        todo!();
    }
}

impl Drop for TaskRegistrySqlite {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::core::core_types::{TaskDefinition, TaskState, TaskStatus};
    use crate::registry::task_registry::TaskRegistry;
    use crate::registry::task_registry_sqlite::TaskRegistrySqlite;

    #[test]
    fn get_and_write_to_database() {
        let task_definition1 = TaskDefinition {
            message: "hello from task 1".to_string(),
            sleep_time_seconds: 4,
        };
        let task_definition2 = TaskDefinition {
            message: "hello from task 2".to_string(),
            sleep_time_seconds: 6,
        };
        let mut registry = TaskRegistrySqlite::new();
        let task1_id = "Task 1";
        let task2_id = "Task 2";
        registry.create_task("Task 1", &task_definition1);
        registry.create_task("Task 2", &task_definition2);
        let retrieved_task1 = registry.get_task(task1_id).unwrap();
        let retrieved_task2 = registry.get_task(task2_id).unwrap();
        assert_eq!(TaskState::new(task1_id, &task_definition1), retrieved_task1);
        assert_eq!(TaskState::new(task2_id, &task_definition2), retrieved_task2);
    }
}
