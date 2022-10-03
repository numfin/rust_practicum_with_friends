use std::{any::Any, fmt::format};

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};

use serde::Deserialize;

use crate::{
    domains::todo::{models, repositories},
    modules::AppData,
};

#[post("")]
pub async fn create_todo(app_data: Data<AppData>) -> HttpResponse {
    let todo_col = app_data
        .db_conn
        .db
        .collection::<repositories::todo::Todo>("todos");
    let todos = {
        use fake::Fake;
        (0..5)
            .map(|_| {
                let random_title: String = fake::faker::lorem::en::Sentence(5..10).fake();
                repositories::todo::Todo::new(random_title)
            })
            .collect::<Vec<_>>()
    };
    match todo_col.insert_many(todos, None).await {
        Ok(op_result) => {
            let ids: Vec<String> = op_result
                .inserted_ids
                .into_iter()
                .map(|(_, body)| body.as_object_id().unwrap().to_hex())
                .collect();
            HttpResponse::Ok().json(ids)
        }
        Err(err) => HttpResponse::BadGateway().body(err.to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub name: String,
    pub status: models::TodoStatus,
}
