// events/server_print.rs
use std::error::Error;

struct ServerPrintHandler<A>(fn(&[&dyn Any], SocketRef<A>) -> Result<(), Box<dyn Error>>);

impl<A> MessageHandler<A, ()> for ServerPrintHandler<A> {
    fn call(&self, args: &[&dyn Any], socket: SocketRef<A>) -> Result<(), Box<dyn Error>> {
        (self.0)(args, socket)
    }
}

pub fn handler<A>() -> ServerPrintHandler<A> {
    ServerPrintHandler(|_args: &[&dyn Any], _socket: SocketRef<A>| {
        println!("Server console print received from client");
        Ok(())
    })
}
