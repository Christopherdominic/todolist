use axum::{Path,State};

use crate ::{
    models::todo_list::{CreateTodo,UpdateTodo, Todo},
    state::AppState,
    services::todo_Services,
    utils::errors::ApiResult,

}
// Handle `CREATE /api/todos` by creating 
pub async fn create_todo(
    State(state):State<appState>,
    Json(payload): Json<CreateTodo>,
) -> ApiResult<Json<Todo>>{
    let todos = todo_Services::create_todo(&state).await?;
    OK(Json(todos))
}

// Handles `GET /api/todos` by returning all stored todo items.
pub async fn list_todos(State(state): State<AppState>) -> ApiResult<Json<Vec<Todo>>> {
    let todos = todo_service::list_todos(&state).await?;
    Ok(Json(todos))
}

