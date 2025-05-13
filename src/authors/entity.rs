use serde::Serialize;
use sqlx::prelude::FromRow;
use time::Date;

#[derive(FromRow, Serialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<Date>,
}
