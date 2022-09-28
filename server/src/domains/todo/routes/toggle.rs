use crate::{domains::todo::models, modules::AppData};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};

pub async fn toggle_todo(app_data: Data<AppData>, id: Path<String>) -> impl Responder {
    let status = todo::TodoStatus::Done;
    let body = models::Todo {
        id: id.to_string(),
        status: models::TodoStatus::InProgress {
            from: "kek".to_string(),
        },
        name: "name".to_string(),
    };

    HttpResponse::Ok().json(body)
}
