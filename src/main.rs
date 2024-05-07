use axum::routing::get;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
    });

    let app = axum::Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}