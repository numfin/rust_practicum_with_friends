use actix_web::{
    web::{self},
    Scope,
};
mod models;
mod routes;

pub fn scope() -> Scope {
    Scope::new("todo")
        .route("", web::get().to(routes::list::list_todos))
        .route("{id}", web::put().to(routes::toggle::toggle_todo))
        .route("", web::post().to(routes::create::create_todo))
}
