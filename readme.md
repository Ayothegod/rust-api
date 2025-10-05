// NOTE: projects to build
// URL Shortener — POST a URL, get a short code, redirect on GET.
// Book API — list books, add book, delete book (good for testing Path, Query, Json).

// NOTE: rust specific
// Ownership & Borrowing — essential for APIs where data is passed between handlers.
// Structs & Enums — model your request/response data.
// Traits & Derive Macros — e.g., Serialize, Deserialize for JSON.
// Result & Option — error handling for routes.
// Concurrency — tokio async/await, Arc<Mutex<T>> for shared state.

// NOTE: axum specific
// Routing — Router::new().route(...).
// Extractors — getting JSON, query params, path params.
// Responses — returning JSON, HTML, status codes.
// Middleware — logging, error handling, authentication.
// State Management — using axum::extract::State with Arc.


2025-09-24T12:13:04.725169Z DEBUG request{method=GET uri=/todos version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200

2025-09-24T12:17:11.434514Z DEBUG request{method=GET uri=/todos matched_path="/todos"}: tower_http::trace::on_response: finished processing request latency=0 ms status=200