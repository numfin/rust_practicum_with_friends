use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use config::Config;
use modules::{mongo::MongoConnection, AppData};

mod config;
mod domains;
pub mod logs;
mod modules;

pub async fn create_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let db_conn = MongoConnection::connect(&config.db.url, &config.db.name).await?;
    let app_data = Data::new(AppData {
        db_conn: Arc::new(db_conn),
        config,
    });

    let addr = ("0.0.0.0", 8080);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(domains::init_resources)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
