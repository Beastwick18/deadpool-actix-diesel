use std::fmt::Display;

use actix_web::web;
use deadpool::managed::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use serde::Serialize;
use utoipa::ToSchema;

pub mod diesel;
pub mod postgres;

#[derive(Clone)]
pub struct AppData {
    pub diesel: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    data: Option<T>,
    message: String,
    code: i32,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn data(data: T, message: impl Display, code: i32) -> ApiResponse<T> {
        Self {
            data: Some(data),
            message: message.to_string(),
            code,
        }
    }

    pub fn empty(message: impl Display, code: i32) -> ApiResponse<T> {
        Self {
            data: None,
            message: message.to_string(),
            code,
        }
    }

    pub fn json(self) -> web::Json<ApiResponse<T>> {
        web::Json(self)
    }
}
