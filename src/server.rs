use axum::{
    Router,
    routing::{get, get_service},
};

use tower_http::services::ServeDir;

use crate::network::get_host_ip;

#[tokio::main]
pub async fn serve(root_dir: &str) {
    let app = Router::new()
        .route("/", get(|| async { "Ok!" }))
        .nest_service(
            "/shows",
            get_service(ServeDir::new(root_dir)).handle_error(|error| async move {
                println!("Error serving directory: {}", error);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong",
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // println!("Serving directory {}", root_dir);
    let ip = get_host_ip().expect("Failed to get local IP");
    // println!("Serving on http://{}:8080/shows", ip);

    axum::serve(listener, app).await.unwrap();
}
