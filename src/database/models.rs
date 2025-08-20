use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use crate::database::schema::users;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub ssn: String,
    pub email: Option<String>,
}
