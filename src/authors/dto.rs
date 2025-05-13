use garde::Validate;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use time::Date;

#[derive(Deserialize, Validate)]
pub struct CreateAuthorDto {
    #[garde(length(min = 2, max = 60))]
    pub name: String,

    #[garde(email)]
    pub email: String,

    #[garde(skip)]
    pub date_of_birth: Option<Date>,
}

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateAuthorDto {
    #[garde(length(min = 2, max = 60))]
    pub name: Option<String>,

    #[garde(email)]
    pub email: Option<String>,

    #[garde(skip)]
    pub date_of_birth: Option<Date>,
}
