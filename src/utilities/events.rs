/*!
 * Horizon Child Server
 *
 * This server software is part of a distributed system designed to facilitate communication
 * and data transfer between multiple child servers and a master server. Each child server
 * operates within a "Region map" managed by the master server, which keeps track of their
 * coordinates in a relative cubic light-year space. The coordinates are stored in 64-bit floats
 * to avoid coordinate overflow and to ensure high precision.
 *
 * The system works as follows:
 *
 * 1. Event Handling:
 *    - The child server receives events from the master server. Each event contains its origin,
 *      data, and a propagation distance, which determines how far the event should spread.
 *
 * 2. Event Propagation:
 *    - Upon receiving an event, the child server calculates which neighboring child servers
 *      should receive the event based on the event's origin and the specified propagation distance.
 *    - This calculation considers all adjacent coordinates within a 3x3x3 cube centered on the
 *      server's coordinate, ensuring that all relevant neighbors are included.
 *
 * 3. Event Transmission:
 *    - After determining the target neighbors, the child server sends the event to the master server.
 *      The master server then multicasts the event to the appropriate neighboring child servers.
 *
 * 4. Coordinate Management:
 *    - Each child server maintains its position in the region map, identified by a coordinate
 *      (x, y, z). The coordinates are managed as integers, representing the position in the cubic
 *      light-year space.
 *    - The child server can calculate the network address of neighboring servers based on their
 *      coordinates, allowing for direct communication.
 *
 * The key components of this system include:
 *
 * - Event: Represents an event with an origin, data, and propagation distance.
 * - Coordinate: Represents a position in the region map.
 * - ChildServer: Represents a child server with methods to receive, handle, and send events.
 *
 * The `ChildServer` struct contains methods for:
 * - Initializing the server with its ID, coordinate, parent server address, and local address.
 * - Receiving events from the master server.
 * - Determining which neighboring servers should receive an event.
 * - Sending events to the parent server for further multicast.
 * - Running the server to continuously listen for and handle events.
 *
 * Usage:
 * - The child server is initialized with a unique ID, its coordinate, the master server's address,
 *   and its own local address.
 * - The server then enters a loop, continuously receiving and handling events.
 *
 * This implementation uses `serde` and `bincode` crates for serialization and deserialization of
 * events to ensure efficient data transfer.
 */

use std::net::{UdpSocket, SocketAddr};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    origin: (f64, f64, f64),
    data: String,
    propagation_distance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

struct ChildServer {
    id: u64,
    coordinate: Coordinate,
    parent_addr: SocketAddr,
    socket: UdpSocket,
}

impl ChildServer {
    fn new(id: u64, coordinate: Coordinate, parent_addr: SocketAddr, local_addr: SocketAddr) -> Self {
        let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");
        ChildServer {
            id,
            coordinate,
            parent_addr,
            socket,
        }
    }

    fn receive_event(&self) -> Event {
        let mut buf = [0u8; 1024];
        let (amt, _src) = self.socket.recv_from(&mut buf).expect("Didn't receive data");
        let event: Event = bincode::deserialize(&buf[..amt]).expect("Failed to deserialize event");
        event
    }

    fn determine_propagation(&self, event: &Event) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        let max_distance = event.propagation_distance;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let neighbor = Coordinate {
                        x: self.coordinate.x + x,
                        y: self.coordinate.y + y,
                        z: self.coordinate.z + z,
                    };

                    let distance = ((neighbor.x as f64 - event.origin.0).powi(2)
                        + (neighbor.y as f64 - event.origin.1).powi(2)
                        + (neighbor.z as f64 - event.origin.2).powi(2))
                        .sqrt();

                    if distance <= max_distance {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }

    fn send_event(&self, event: &Event, target: &Coordinate) {
        let msg = bincode::serialize(event).expect("Failed to serialize event");
        let addr = self.calculate_addr(target);
        self.socket.send_to(&msg, addr).expect("Failed to send event");
    }

    fn calculate_addr(&self, target: &Coordinate) -> SocketAddr {
        // Implement your logic to calculate the socket address of the target coordinate
        // This is a placeholder, you need to provide the actual mapping from coordinate to address
        SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)
    }

    fn handle_event(&self, event: Event) {
        let neighbors = self.determine_propagation(&event);

        for neighbor in neighbors {
            self.send_event(&event, &neighbor);
        }
    }

    fn run(&self) {
        loop {
            let event = self.receive_event();
            self.handle_event(event);
        }
    }
}

fn main() {
    let id = 0;
    let coordinate = Coordinate { x: 0, y: 0, z: 0 };
    let parent_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let local_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();

    let server = ChildServer::new(id, coordinate, parent_addr, local_addr);
    server.run();
}
