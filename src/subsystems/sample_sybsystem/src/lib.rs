use socketioxide::extract::{SocketRef, Data};

pub fn init(socket: SocketRef) {
    println!("A new client with ID {} has joined the game!", socket.id.to_string());
}


////////////////////////////////////////////
// BELOW THIS IS THE DEFAULT EXAMPLE CODE //
////////////////////////////////////////////

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
