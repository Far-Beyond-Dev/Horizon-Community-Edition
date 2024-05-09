// events/log.rs
use std::error::Error;
use std::any::Any;
use socketioxide::extract::SocketRef;
use socketioxide::handler::MessageHandler;

struct LogHandler<A>(fn(&[&dyn Any], SocketRef<A>) -> Result<(), Box<dyn Error>>);

impl<A> MessageHandler<A, String> for LogHandler<A> {
    fn call(&self, args: &[&dyn Any], socket: SocketRef<A>) -> Result<(), Box<dyn Error>> {
        (self.0)(args, socket)
    }
}

pub fn handler<A>() -> LogHandler<A> {
    LogHandler(|args: &[&dyn Any], socket: SocketRef<A>| {
        if let Some(msg) = args.get(0).and_then(|arg| arg.downcast_ref::<String>()) {
            println!("Received log message from client: {}", msg);
            (*socket).emit("message-back", msg).ok();
        }
        Ok(())
    })
}