use actix_web::{
    web::{self},
    Scope,
};
mod models;
mod repositories;
mod routes;

pub fn scope() -> Scope {
    Scope::new("todo")
        .route("", web::get().to(routes::list::list_todos))
        .route("{id}", web::put().to(routes::toggle::toggle_todo))
        .service(routes::create::create_todo)
}
