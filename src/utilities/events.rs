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
