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

fn extract_string(value: &sqlite::Value) -> String {
    match &value {
        sqlite::Value::String(i) => i.to_string(),
        _ => panic!(),
    }
}

fn extract_i64(value: &sqlite::Value) -> i64 {
    match &value {
        sqlite::Value::Integer(i) => *i,
        _ => panic!(),
    }
}

pub struct TaskRegistrySqlite {
    connection: sqlite::Connection,
}

impl TaskRegistrySqlite {
    pub fn new() -> TaskRegistrySqlite {
        let query = format!("CREATE TABLE {TABLE_NAME} (status TEXT, name TEXT PRIMARY KEY, sleep_time_seconds INTEGER, message TEXT);");
        let connection = sqlite::Connection::open(":memory:").unwrap();
        connection.execute(query).unwrap();
        TaskRegistrySqlite { connection }
    }
}

impl task_registry::TaskRegistry for TaskRegistrySqlite {
    fn get_task(self: &Self, task_id: &str) -> Result<TaskState, task_registry::TaskNotFoundError> {
        let query = format!("SELECT * FROM {TABLE_NAME} WHERE name = ?");
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, task_id)).unwrap();
        let mut cursor = statement.iter();
        let optional_values = cursor.try_next().unwrap();
        if let Some(values) = optional_values {
            let serialised_task_state: SerialisedTaskState = (
                extract_string(&values[0]),
                extract_string(&values[1]),
                extract_i64(&values[2]),
                extract_string(&values[3]),
            );
            Ok(deserialise_task_state(serialised_task_state))
        } else {
            Err(task_registry::TaskNotFoundError {
                task_id: task_id.to_string(),
            })
        }
    }

    fn update_task_from_control_loop(&mut self, task_id: &str, status: TaskStatus) {
        let query = format!("UPDATE {TABLE_NAME} SET status = :status WHERE name = :name");
        let mut statement = self.connection.prepare(query).unwrap();
        statement
            .bind_iter::<_, (_, sqlite::Value)>([
                (":status", status.to_string().into()),
                (":name", task_id.into()),
            ])
            .unwrap();
        let state = statement.next().unwrap();
        assert_eq!(state, sqlite::State::Done);
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
    ) -> Box<dyn Iterator<Item = TaskState> + 'a> {
        let statuses_vec = Vec::from_iter(statuses.into_iter().map(|x| x.to_string()));
        let question_marks =
            Vec::from_iter(statuses.into_iter().map(|_x| "?".to_string())).join(", ");
        let query = format!("SELECT * FROM {TABLE_NAME} WHERE status in ({question_marks})");
        println!("{}", &query);
        let mut statement = self.connection.prepare(query).unwrap();
        statement
            .bind_iter::<_, (usize, sqlite::Value)>(
                statuses_vec
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (i + 1, sqlite::Value::String(x.to_string()))),
            )
            .unwrap();
        let cursor = statement.iter();
        let my_iter = cursor.map(|row_result| {
            let row = row_result.unwrap();
            let values = Vec::<sqlite::Value>::from(row);
            let serialised_task_state: SerialisedTaskState = (
                extract_string(&values[0]),
                extract_string(&values[1]),
                extract_i64(&values[2]),
                extract_string(&values[3]),
            );
            deserialise_task_state(serialised_task_state)
        });
        Box::new(my_iter.collect::<Vec<_>>().into_iter())
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
    use std::collections::HashSet;

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
        registry.create_task(task1_id, &task_definition1);
        registry.create_task(task2_id, &task_definition2);
        let retrieved_task1 = registry.get_task(task1_id).unwrap();
        let retrieved_task2 = registry.get_task(task2_id).unwrap();
        assert_eq!(TaskState::new(task1_id, &task_definition1), retrieved_task1);
        assert_eq!(TaskState::new(task2_id, &task_definition2), retrieved_task2);
    }

    #[test]
    fn update_element_in_database() {
        let task_definition = TaskDefinition {
            message: "hello from task 1".to_string(),
            sleep_time_seconds: 4,
        };
        let mut registry = TaskRegistrySqlite::new();
        let task_id = "my task";
        registry.create_task(task_id, &task_definition);
        let retrieved_task = registry.get_task(task_id).unwrap();
        assert_eq!(retrieved_task.status, TaskStatus::PENDING);
        registry.update_task_from_control_loop(task_id, TaskStatus::RUNNING);
        let retrieved_task_after_update = registry.get_task(task_id).unwrap();
        assert_eq!(retrieved_task_after_update.status, TaskStatus::RUNNING);
    }

    #[test]
    fn list_by_statuses() {
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
        registry.create_task(task1_id, &task_definition1);
        registry.create_task(task2_id, &task_definition2);
        let mut statuses = HashSet::new();
        statuses.insert(TaskStatus::PENDING);
        let tasks_iter = registry.get_tasks(&statuses);
        let mut tasks = Vec::from_iter(tasks_iter);
        tasks.sort_by(|a, b| a.name.to_string().cmp(&b.name));
        let expected_tasks = vec![
            TaskState::new(task1_id, &task_definition1),
            TaskState::new(task2_id, &task_definition2),
        ];
        assert_eq!(tasks, expected_tasks);
    }
}
