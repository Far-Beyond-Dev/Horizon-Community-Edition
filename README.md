# Horizon
An easily scalable game server

```RUST
use tracing::info;
 Helpes append .route to a specific **mutable** variable so you don't have to.

 The first parameter is the variable itself; the value of `axum::Router::new()`.
 The second value is the endpoint, eg "/".
 The third value is what to return back.

 # Example

 ```rust
 let mut app = axum::Router::new()
 .layer(layer);

 define_routes!(app,
                "/", "Hello, World!",
                "/goodbye", "Goodbye, World!",
                "/function", function());
 ```