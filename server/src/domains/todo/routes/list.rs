use actix_web::{web::Data, Responder};

use crate::modules::AppData;

pub async fn list_todos(app_data: Data<AppData>) -> impl Responder {
    vec![]
}
