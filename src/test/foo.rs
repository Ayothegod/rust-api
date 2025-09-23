use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct AppState {
    auth_time: u32,
}

#[tokio::main]
async fn main() {
    // let shared_state = Arc::new(Mutex::new(AppState {authTime: 1}))
    let shared_state = Arc::new(AppState { auth_time: 1 });

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .with_state(shared_state);
    // .route("/foo/bar", get(foo_bar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    // println!("Hello bpi");
    Json(json!("hello"))
}

#[derive(Deserialize)]
struct UserData {
    name: String,
    email: String,
}

async fn post_foo(Json(payload): Json<UserData>) -> Json<Value> {
    println!(
        "Received a request for: {} with email: {}",
        payload.name, payload.email
    );

    Json(json!({"msg":"User data created", "email": payload.email}))
}

async fn get_foo(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    for (key, value) in &params {
        println!("{key}: {value}");
    }
    Json(json!({"userid": params}))
}


// axum = { path = "../../axum" }
// serde = { version = "1.0", features = ["derive"] }
// tokio = { version = "1.0", features = ["full"] }
// tower = { version = "0.5.2", features = ["util", "timeout"] }
// tower-http = { version = "0.6.1", features = ["add-extension", "trace"] }
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }
// uuid = { version = "1.0", features = ["serde", "v4"] }


// use axum::{extract::State, routing::post, Router, Json};
// use serde::{Deserialize, Serialize};
// use std::{sync::{Arc, Mutex}, net::SocketAddr};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Todo {
//     id: u64,
//     title: String,
// }

// #[derive(Clone)]
// struct AppState {
//     todos: Arc<Mutex<Vec<Todo>>>,
// }

// async fn add_todo(
//     State(state): State<AppState>,
//     Json(new_todo): Json<Todo>,
// ) -> Json<Vec<Todo>> {
//     // Lock before modifying
//     let mut todos = state.todos.lock().unwrap();
//     todos.push(new_todo);
//     Json(todos.clone())
// }

// #[tokio::main]
// async fn main() {
//     let state = AppState {
//         todos: Arc::new(Mutex::new(Vec::new())),
//     };

//     let app = Router::new()
//         .route("/todos", post(add_todo))
//         .with_state(state);

//     let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
