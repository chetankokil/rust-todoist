use crate::{errors::AppError, models};
use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "select email, password from user where email = $!",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    let result = sqlx::query("insert into users(email, password) values($1,$2)")
        .bind(&credentials.email)
        .bind(&credentials.password)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({"msg" : "registered successfully"})))
    }
}
