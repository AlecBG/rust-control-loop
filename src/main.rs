use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::mpsc;
use task_runner::control::control_api::{add_task, get_task, ControlApi};
use task_runner::control::control_loop::ControlLoop;
use task_runner::core::core_types::NewTaskInfo;
use task_runner::registry::task_registry::TaskRegistry;
use task_runner::registry::task_registry_sqlite::{TablePermanance, TaskRegistrySqlite};

const DATABASE_NAME: &str = "test.db";

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

fn registry_factory() -> Box<dyn TaskRegistry> {
    Box::new(TaskRegistrySqlite::new(
        DATABASE_NAME,
        "test_table",
        TablePermanance::DropOnClose,
    ))
}

#[actix_web::main]
async fn server_main(sender: mpsc::Sender<NewTaskInfo>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let control_api = ControlApi::new(sender.clone(), registry_factory);
        let data = Data::new(control_api);
        App::new()
            .app_data(data.clone())
            .service(add_task)
            .service(get_task)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

fn main() {
    let (sender, receiver) = mpsc::channel::<NewTaskInfo>();

    // Run server in background thread
    let server_handle = std::thread::spawn(move || {
        server_main(sender.clone()).unwrap();
    });
    // Run control loop for two minutes
    let registry =
        TaskRegistrySqlite::new(DATABASE_NAME, "test_table", TablePermanance::DropOnClose);
    let mut control_loop = ControlLoop::new(&registry, receiver);
    for _ in 0..120 {
        control_loop.run_once();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    // Stop server
    server_handle.join().unwrap();
}
