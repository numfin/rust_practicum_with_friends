use actix_web::web::ServiceConfig;
mod todo;

pub fn init_resources(cfg: &mut ServiceConfig) {
    cfg.service(todo::scope());
}
