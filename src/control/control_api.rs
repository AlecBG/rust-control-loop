use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::mpsc::Sender;

use crate::core::core_types::NewTaskInfo;
use crate::models::tasks::{CreateTaskDefinitionResponse, TaskDefinitionModel, TaskStateModel};
use crate::registry::task_registry::TaskRegistry;

pub struct ControlApi {
    sender: Sender<NewTaskInfo>,
    registry: Box<dyn TaskRegistry>,
}

impl ControlApi {
    pub fn new(
        sender: Sender<NewTaskInfo>,
        registry_factory: fn() -> Box<dyn TaskRegistry>,
    ) -> ControlApi {
        ControlApi {
            sender,
            registry: registry_factory(),
        }
    }
}

#[post("/tasks/{task_id}")]
pub async fn add_task(
    task_id: web::Path<String>,
    task: web::Json<TaskDefinitionModel>,
    control_api: web::Data<ControlApi>,
) -> impl Responder {
    println!("Adding task {:?}", task);
    let task_definition_model = task.into_inner();
    let task_definition = (&task_definition_model).clone().create_task_definition();

    let new_task_info = NewTaskInfo {
        task_id: task_id.to_string(),
        task_definition: (&task_definition).clone(),
    };
    control_api.sender.send(new_task_info).unwrap();
    let response = CreateTaskDefinitionResponse {
        task_id: task_id.to_string(),
        task_definition: task_definition_model,
    };
    HttpResponse::Ok().json(response)
}

#[get("/tasks/{task_id}")]
pub async fn get_task(
    task_id: web::Path<String>,
    control_api: web::Data<ControlApi>,
) -> impl Responder {
    println!("Getting task {:?}", task_id.to_string());
    let task_state = control_api.registry.get_task(&task_id.to_string());
    match task_state {
        Ok(task_state) => HttpResponse::Ok().json(TaskStateModel::from_task_state(&task_state)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
