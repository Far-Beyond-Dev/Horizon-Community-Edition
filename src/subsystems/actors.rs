use socketioxide::extract::{Bin, Data, SocketRef};
use serde_json::{json, Value};
use tracing::info;

use crate::structs::*;

pub fn init (socket: SocketRef) {
    socket.on(
        "spawnActor",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            info!("Todo: implement actor spawning");

            // This is where data will be added to pebbleVault; our high performance in-memory geo-indexed database

            socket.emit("return_spawnActor", 200).ok();
        },
    );
}