use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;

struct AppData {
    start_time: Instant,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[derive(Serialize)]
pub struct HealthcheckData {
    status: String,
    uptime: u128,
}

#[get("/healthcheck")]
async fn healthcheck(data: Data<Mutex<AppData>>) -> impl Responder {
    let local_data = data.lock().unwrap();
    web::Json(HealthcheckData {
        status: "healthy".to_string(),
        uptime: Instant::now()
            .duration_since(local_data.start_time)
            .as_millis(),
    })
}

pub fn init(config: &mut web::ServiceConfig) {
    let data = Data::new(Mutex::new(AppData {
        start_time: Instant::now(),
    }));
    config.app_data(data);
    config.service(web::scope("").service(index).service(healthcheck));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
