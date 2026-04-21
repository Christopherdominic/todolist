use axum:: {routing::{get, post}, Router};

use crate::{
    controllers::todo.controllers::{create_todo,list_todo,update_todo},
    state::AppState,
};

// create CRUD route for todo services

pub fn todo_routes() -> Router<AppState>{
    Router::new()
    .route("/todos",post(create_todo)
    .get(list_todos))
    
}