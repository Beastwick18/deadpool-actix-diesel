use actix_web::{Responder, get, web};

#[get("/api/Diesel/GetRow")]
async fn get_row() -> impl Responder {
    "static"
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_row);
}
