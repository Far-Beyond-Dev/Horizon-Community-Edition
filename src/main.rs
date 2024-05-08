use axum::routing::get;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use tracing::info;

/// Helpes append .route to a specific **mutable** variable so you don't have to.
///
/// The first parameter is the variable itself; the value of `axum::Router::new()`.
/// The second value is the endpoint, eg "/".
/// The third value is what to return back.
///
/// # Example
///
/// ```rust
/// let mut app = axum::Router::new()
/// .layer(layer);
///
/// define_routes!(app,
///                "/", "Hello, World!",
///                "/goodbye", "Goodbye, World!",
///                "/function", function());
/// ```
macro_rules! define_routes {
     ($app:expr, $($path:expr, $handler:expr),* $(,)?) => {
        $(
            $app = $app.route($path, get(|| async { $handler }));
        )*
     };
}

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

    let mut app = axum::Router::new()
    .layer(layer);

    define_routes!(app,
                   "/", "Hello, World!");

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}
