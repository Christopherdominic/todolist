use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::todo::TodoCreatedResponse;

// Global application state shared with request handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub todos: Arc<RwLock<Vec<TodoCreatedResponse>>>,
}


impl AppState {
   pub fn new(db: PgPool) -> Self {
      Self {
          db,
          todos: Arc::new(RwLock::new(Vec::new())),
      }
    }
}