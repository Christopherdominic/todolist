use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{
    models::todo::{CreateTodoRequest, TodoCreatedResponse},
    state::AppState,
};

// TODO routes mounted under /api.
pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
}

async fn list_todos(
    State(state): State<AppState>,
) -> Json<Vec<TodoCreatedResponse>> {
    let todos = state.todos.read().await.clone();
    Json(todos)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> (StatusCode, Json<TodoCreatedResponse>) {
    if payload.title.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(TodoCreatedResponse {
                id: String::new(),
                title: String::new(),
                description: Some("title must not be empty".to_string()),
                status: "error".to_string(),
            }),
        );
    }
    let todo = TodoCreatedResponse {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
        description: payload.description,
        status: "pending".to_string(),
    };

    state.todos.write().await.push(todo.clone());

    (StatusCode::CREATED, Json(todo))
}
