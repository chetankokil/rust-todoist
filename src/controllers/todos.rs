use axum::{Json, Extension};
use sqlx::PgPool;

use crate::{errors::AppError, models::{self, todos::Todos}};

pub async fn get_all(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Todos>>, AppError> {

    let todos = sqlx::query_as::<_, models::todos::Todos>("select * from todos")
                                .fetch_all(&pool)
                                .await
                                .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(todos)) 
}