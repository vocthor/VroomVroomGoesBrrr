use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};


pub fn start_web_server(port: u16) {
    tokio::spawn(async move{
        start(port).await;
    });
}

async fn start(port: u16) {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello_handler));

    let addr = format!("localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "Hello, world!"
}
