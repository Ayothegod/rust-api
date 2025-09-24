pub mod todo;

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{HeaderValue, StatusCode},
    routing::{get, patch},
};
use serde::Serialize;
use std::{sync::atomic::AtomicU32, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;


pub fn create_router() -> Router {
    
    // let db = todo::Db::default();
    let state = AppState::default();

    Router::new()
        .route("/todos", get(todo::todos_index).post(todo::todos_create))
        .route(
            "/todos/{id}",
            patch(todo::todos_update).delete(todo::todos_delete),
        )
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
                )
                .into_inner(),
        )
        .with_state(state)
}

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

#[derive(Default, Clone)]
pub struct AppState {
    db: Arc<RwLock<HashMap<Uuid, Todo>>>,
    no_of_users: Arc<AtomicU32>
}