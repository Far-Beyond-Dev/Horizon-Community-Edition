#[macro_use]
mod export {
    /// Append .on to a specific **mutable** variable so you don't have to.
    ///
    /// The first parameter is the variable itself; a socketioxide::extract::SocketRef type.
    /// The second value is the endpoint, eg "/".
    /// The third value is what to return back.
    ///
    /// # Example
    ///
    /// ```rust
    /// define_event!(socket, // where socket is a socketioxide::extract::SocketRef type
    ///               "/", hello_world(),
    ///               "/function", function());
    /// ```
    ///
    /// Note: this example does not compile becuase you might not have a function literally named
    /// function() and no variable with a SocketRef type. This is just an example.
    #[macro_export]
    macro_rules! define_event {
         ($app:expr, $($path:expr, $handler:expr),* $(,)?) => {
            $(
                $app.on($path, move || { $handler });
            )*
         };
    }
}
