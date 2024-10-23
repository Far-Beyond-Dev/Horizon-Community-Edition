/// Append .on to a **mutable** SocketRef to keep everything concise.
///
/// The first parameter is the variable itself; a socketioxide::extract::SocketRef type.
/// The second value is the endpoint, eg "/".
/// The third value is the function to be called.
///
/// # Example
///
/// ```rust
/// define_event!(socket, // where socket is a socketioxide::extract::SocketRef type
///               "/", hello_world(),
///               "/function", function());
/// ```
///
/// Note: this example does not compile because you may not have such variables or functions.
/// This is just an example of how it's meant to be used.
#[macro_export]
macro_rules! define_event {
     ($app:expr, $($path:expr, $handler:expr),* $(,)?) => {
        $(
            {
                let app = &$app;  // Borrow $app
                let handler = $handler.clone(); //define handler within the macro's scope
                app.on($path, move || { handler });
            }
        )*
     };
}