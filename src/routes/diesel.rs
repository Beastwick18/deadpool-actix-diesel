use actix_web::{get, web};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    database::{models::User, schema::users},
    routes::{ApiResponse, AppData},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

const DIESEL: &str = "diesel";

#[utoipa::path(tag = DIESEL, responses((status = 200, description = "Return list of users", body = ApiResponse::<Vec<User>>)))]
#[get("/Diesel/GetRow")]
async fn get_row(data: web::Data<AppData>) -> web::Json<ApiResponse<Vec<User>>> {
    let mut conn = data.diesel.get().await.unwrap();
    let results = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    ApiResponse::data(results, "OK".to_string(), 200).json()
}

#[utoipa::path(tag = DIESEL, responses((status = 200, description = "Return \"static\"")))]
#[get("/Diesel/AddSampleRow")]
async fn add_sample_row(data: web::Data<AppData>) -> web::Json<ApiResponse<i32>> {
    let mut conn = data.diesel.get().await.unwrap();
    let new_row = (
        users::first_name.eq("Steven"),
        users::last_name.eq("Culwell"),
        users::ssn.eq("123-12-1234"),
    );

    new_row
        .insert_into(users::table)
        .execute(&mut conn)
        .await
        .unwrap();

    ApiResponse::empty("OK", 200).json()
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_row);
    cfg.service(add_sample_row);
}
