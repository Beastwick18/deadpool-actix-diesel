use actix_web::{Responder, get};
use utoipa_actix_web::service_config::ServiceConfig;

const POSTGRES: &str = "postgres";

#[utoipa::path(tag = POSTGRES, responses(
    (status = 200, description = "Return \"static\"")
))]
#[get("/Postgres/GetRow")]
async fn get_row() -> impl Responder {
    "static"
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_row);
}
