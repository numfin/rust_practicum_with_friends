use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder};
use env_logger;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, app_data: Data<AppData>) -> impl Responder {
    let db_conn = app_data.db_conn.clone();
    let data = db_conn.get_data().await;
    format!("Hello {name}. With {data}!")
}

pub struct AppData {
    pub db_conn: Arc<MongoConnection>,
}

pub async fn create_server() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db_conn = MongoConnection::connect().await;
    let app_data = Data::new(AppData {
        db_conn: Arc::new(db_conn),
    });

    let addr = ("0.0.0.0", 8080);
    HttpServer::new(move || {
        let app = App::new().app_data(app_data.clone()).service(greet);
        app
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

pub struct MongoConnection;
impl MongoConnection {
    pub async fn connect() -> Self {
        Self
    }
    pub async fn get_data(&self) -> String {
        "data".to_string()
    }
}
