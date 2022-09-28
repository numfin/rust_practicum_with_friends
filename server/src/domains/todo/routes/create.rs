use actix_web::{post, web, Responder};
use serde::Deserialize;

use crate::domains::todo::models;

pub async fn create_todo(req: web::Json<Request>) -> impl Responder {
    ""
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub name: String,
    pub status: models::TodoStatus,
}
