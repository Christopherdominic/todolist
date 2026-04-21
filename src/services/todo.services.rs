use chrono::Utc;
use uuid::Uuid;

use crate::{
    models::todo::{CreateTodoRequest, Todo, TodoStatus, UpdateTodoRequest},
    state::AppState,
    utils::{errors::{ApiError, ApiResult}, id::new_todo_id},
};

/// Creates and stores a new todo record.
pub async fn create_todo(state: &AppState, payload: CreateTodoRequest) -> ApiResult<Todo> {
    if payload.title.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "title must not be empty".to_string(),
        ));
    }

    let now = Utc::now();
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (id, title, description, status, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(new_todo_id())
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(TodoStatus::Pending)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await?;

    Ok(todo)
}

/// Returns all todos ordered by creation time.
pub async fn list_todos(state: &AppState) -> ApiResult<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos ORDER BY created_at ASC",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(todos)
}

/// Returns a single todo by ID or a `NotFound` error.
pub async fn get_todo(state: &AppState, id: Uuid) -> ApiResult<Todo> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("todo {id} not found")))
}

/// Applies partial updates to a todo and returns the updated value.
pub async fn update_todo(
    state: &AppState,
    id: Uuid,
    payload: UpdateTodoRequest,
) -> ApiResult<Todo> {
    if let Some(title) = payload.title.as_deref() {
        if title.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "title must not be empty when provided".to_string(),
            ));
        }
    }

    let existing = get_todo(state, id).await?;

    let new_title = payload.title.unwrap_or(existing.title);
    let new_description = if payload.description.is_some() {
        payload.description
    } else {
        existing.description
    };
    let new_status = payload.status.unwrap_or(existing.status);

    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos
         SET title = $1, description = $2, status = $3, updated_at = $4
         WHERE id = $5
         RETURNING *",
    )
    .bind(&new_title)
    .bind(&new_description)
    .bind(new_status)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(todo)
}

/// Deletes a todo by ID and returns the deleted value.
pub async fn delete_todo(state: &AppState, id: Uuid) -> ApiResult<Todo> {
    sqlx::query_as::<_, Todo>("DELETE FROM todos WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("todo {id} not found")))
}
