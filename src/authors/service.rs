use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use garde::Validate;
use serde_json::{Value, json};
use sqlx::{PgPool, Row};

use super::{
    dto::{CreateAuthorDto, UpdateAuthorDto},
    entity::Author,
};

pub async fn get_all(
    State(db_pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": e.to_string(),
                    "message": "Failed to fetch authors"
                })),
            )
        })?;

    Ok(Json(json!(authors)))
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Validations
    if id < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "ID must be positive i32"})),
        ));
    }

    // Try search by ID
    let author = sqlx::query_as::<_, Author>("SELECT * FROM authors WHERE id = $1")
        .bind(id)
        .fetch_optional(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Database error",
                    "error": e.to_string()
                })),
            )
        })?;

    // Result handler
    match author {
        Some(author) => Ok(Json(json!(author))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"message": "Author not found"})),
        )),
    }
}

pub async fn create(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateAuthorDto>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // Validations
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "errors": validation_errors.to_string(),
            })),
        ));
    }

    // Try save data
    let result = sqlx::query(
        r#"
        INSERT INTO authors(name, email, date_of_birth)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.date_of_birth)
    .fetch_one(&db_pool)
    .await;

    // Result handler
    match result {
        Ok(row) => {
            let id: i32 = row.get("id");
            Ok((
                StatusCode::CREATED,
                Json(json!({
                    "message": "Author created successfully",
                    "id": id
                })),
            ))
        }
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => Err((
            StatusCode::CONFLICT,
            Json(json!({
                "message": "Constraint violation",
                "detail": db_err.to_string()
            })),
        )),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Failed to create author",
                "error": error.to_string()
            })),
        )),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    // Validations
    if id < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Try to delete the record
    let result = sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(id)
        .execute(&db_pool)
        .await;

    // Handle the result
    match result {
        // Success case, exactly 1 record was deleted
        Ok(query_result) if query_result.rows_affected() == 1 => Ok(StatusCode::NO_CONTENT),

        // Success case but no record was found to delete
        Ok(_) => Err(StatusCode::NOT_FOUND),

        // Error due to a database constraint (e.g., referential integrity)
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(StatusCode::CONFLICT)
        }

        // Any other error
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn patch_update(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAuthorDto>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    // Validations
    if id < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "ID must be positive i32"})),
        ));
    }

    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "errors": validation_errors.to_string(),
            })),
        ));
    }

    // Get data if exist
    let existing_author =
        sqlx::query_as::<_, UpdateAuthorDto>("SELECT * FROM authors WHERE id = $1")
            .bind(id)
            .fetch_optional(&db_pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": e.to_string()})),
                )
            })?;

    let existing_author = match existing_author {
        Some(author) => author,
        None => return Err((StatusCode::NOT_FOUND, Json(json!({"message": "Not Found"})))),
    };

    // Update
    let result =
        sqlx::query("UPDATE authors SET name = $1, email = $2, date_of_birth = $3 WHERE id = $4")
            .bind(payload.name.or(existing_author.name))
            .bind(payload.email.or(existing_author.email))
            .bind(payload.date_of_birth.or(existing_author.date_of_birth))
            .bind(id)
            .execute(&db_pool)
            .await;

    match result {
        Ok(query_result) if query_result.rows_affected() == 1 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Failed to update author"})),
        )),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => Err((
            StatusCode::CONFLICT,
            Json(json!({
                "message": "Constraint violation",
                "detail": db_err.to_string()
            })),
        )),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Internal Server Error",
                "error": error.to_string()
            })),
        )),
    }
}
