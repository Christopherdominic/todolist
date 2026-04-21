use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoCreatedResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
