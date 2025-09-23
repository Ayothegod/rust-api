pub mod todo;

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{HeaderValue, StatusCode},
    routing::{get, patch},
};
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    //let db = Db::default();
    let db = todo::Db::default();

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
        .with_state(db)
}
