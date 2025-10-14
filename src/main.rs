// pub mod todos;
// pub mod books_api;
// pub mod error;

// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use crate::books_api::books;

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
//                 format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
//             }),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     // books::check();

//     // Compose the routes
//     let app = todos::create_router();

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

fn main() {
    let text = String::from("The time is 10:00pm");
    println!("Welcome to rust, {text}");

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    match number {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("something else"), // exhaustive wildcard. any other matches
    }
}
