use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    routes::todo::todo_routes,
    state::AppState,
};

/// Builds the application router.
///
/// - Merges health and todo routers.
pub fn build_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api", todo_routes())
        .layer(cors)
        .with_state(state)
}

