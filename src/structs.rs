/////////////////////////////////////////////////////////////////
//                       INFORMATION                                 //
//  This file contains Horizon's global struct definitions.  //
//  Because of this anything that is public in this file      //
//  can be imported by any part of Horizon using           //
//  crate::structs::                                                   //
///////////////////////////////////////////////////////////////
//                    !!!! WARNING !!!!                             //
//  Anything made public in this file *WILL* me           //
//  imported by main.rs                                            //
///////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use socketioxide::extract::SocketRef;
use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    origin: (f64, f64, f64),
    data: String,
    propagation_distance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

pub struct ChildServer {
    id: u64,
    coordinate: Coordinate,
    parent_addr: SocketAddr,
    socket: UdpSocket,
}


///////////////////////////////////////////////////////////////////////////////////////////////////////
// * The `ChildServer` struct contains methods for:                                                       //
// - Initializing the server with its ID, coordinate, parent server address, and local address. //
// - Receiving events from the master server.                                                                //
// - Determining which neighboring servers should receive an event.                                //
// - Sending events to the parent server for further multicast.                                         //
// - Running the server to continuously listen for and handle events.                                //
//////////////////////////////////////////////////////////////////////////////////////////////////////

impl ChildServer {
    pub async fn new(
        id: u64,
        coordinate: Coordinate,
        parent_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> Self {
        let socket = UdpSocket::bind(local_addr).await.expect("Couldn't bind to address");
        ChildServer {
            id,
            coordinate,
            parent_addr,
            socket,
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //                                                      Event Handling:                                                       //
    //  The child server receives events from the master server. Each event contains its origin, data  //
    //  and a propagation distance, which determines how far the event should spread.                    //
    /////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub async fn receive_event(&self) -> Event {
        let mut buf = [0u8; 1024];
        let (amt, _src) = self
            .socket
            .recv_from(&mut buf)
            .await
            .expect("Didn't receive data");
        let event: Event = bincode::deserialize(&buf[..amt]).expect("Failed to deserialize event");
        event
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // * Event Propagation:                                                                                                       //
    // - Upon receiving an event, the child server calculates which neighboring child servers               //
    //    should receive the event based on the event's origin and the specified propagation distance.   //
    // - This calculation considers all adjacent coordinates within a 3x3x3 cube centered on the           //
    //    server's coordinate, ensuring that all relevant neighbors are included.                                  //
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn determine_propagation(&self, event: &Event) -> Vec<Coordinate> {
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


    // TODO: Finish this implementation and move to its own file
    
    //////////////////////////////////////////////////////////////////////////////////////////////////////
    // * Event Transmission:                                                                            //
    // - After determining the target neighbors, the child server sends the event to the master server. //
    // - The master server then multicasts the event to the appropriate neighboring child servers.      //
    //////////////////////////////////////////////////////////////////////////////////////////////////////

    //  `pub async fn send_event(&self, event: &Event, _target: &Coordinate) {
    //      let msg = bincode::serialize(event).expect("Failed to serialize event");
    //      let addr = self.calculate_addr(); // 6/24/2024: removed the `target` parameter due to
    //                                        // compiler errors. it doesn't seem to be used here
    //      self.socket
    //          .send_to(&msg, addr)
    //          .await
    //          .expect("Failed to send event");
    //  }
    //  
    //  pub fn calculate_addr(&self) -> SocketAddr {
    //      // Implement logic to calculate the socket address of the target coordinate
    //      SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)
    //  }
  
    //  pub fn handle_event(&self, event: Event) {
    //      let neighbors = self.determine_propagation(&event);
    //  
    //      for neighbor in neighbors {
    //          self.send_event(&event, &neighbor);
    //      }
    //  }

    //  pub async fn run(&self) {
    //      loop {
    //          let event = self.receive_event().await;
    //          self.handle_event(event);
    //      }
    //  }
}


/////////////////////////////////////////////////////////////////////////////
//                         World object structs:                           //
// These Structs help store an object's location this server's coordanites //
// in the instance grid define a struct for Rotation of objects.           //
/////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct TrajectoryPoint {
    pub accumulated_seconds: f64,
    pub facing: Rotation,
    pub position: Translation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub location: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub translation: Option<Translation>,
    pub scale3D: Scale3D,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            location: None,
            rotation: None,
            scale3D: Scale3D { x: 1.0, y: 1.0, z: 1.0 },
            translation: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveActionValue {
    pub x: f64,
    pub y: f64,
}


////////////////////////////////
//  Define the player struct  //
////////////////////////////////
/// 
#[derive(Debug, Clone)]
pub struct Player {
    // Socket and connection info
    pub socket: SocketRef,
    pub id: String,
    pub last_update: Instant,
    pub is_active: bool,

    // Basic transform data
    pub transform: Option<Transform>,
    pub moveActionValue: Option<MoveActionValue>,
    pub controlRotation: Option<Vec3D>,

    // Motion matching specific data
    pub trajectory_path: Option<Vec<TrajectoryPoint>>,
    pub key_joints: Option<Vec<Vec3D>>,
    pub root_velocity: Option<Vec3D>,

    // Additional data that might be useful
    pub animation_state: Option<String>,
    pub last_input_time: Instant,
}

impl Player {
    pub fn new(socket: SocketRef, id: String) -> Self {
        Player {
            socket,
            id,
            last_update: Instant::now(),
            is_active: true,
            transform: None,
            moveActionValue: None,
            controlRotation: None,
            trajectory_path: None,
            key_joints: None,
            root_velocity: None,
            animation_state: None,
            last_input_time: Instant::now(),
        }
    }

    pub fn update_from_data(&mut self, data: &serde_json::Value) {
        // Implementation of updating player from received data
        // This would be similar to what we did in the update_player_location function
    }
}
pub struct PlayerManager {
    players: Mutex<HashMap<String, Arc<Notify>>>,
}

impl PlayerManager {
    pub fn new() -> Self {
        PlayerManager {
            players: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_player(&self, player_id: String) -> Arc<Notify> {
        let notify = Arc::new(Notify::new());
        self.players.lock().unwrap().insert(player_id, notify.clone());
        notify
    }

    pub fn remove_player(&self, player_id: &str) {
        if let Some(notify) = self.players.lock().unwrap().remove(player_id) {
            notify.notify_one();
        }
    }
}


/////////////////////////////////////////////////////////////////////////////
//                                Save Data                                //
// Save data structs are meant to store the save data of a given planet in //
// memory while it is in use by a client.                                  //
/////////////////////////////////////////////////////////////////////////////

pub struct Chunk {
    id: u32,
    data: str,
}

pub struct Region {
    location: (i64, i64),
    chunks: i64,
}


//////////////////////////////////////////////////////////////////////////
//                               Actor Structs                          //
// The actor structs describee the data attatched to an actor, this may //
// include Location, rotation, scale, meta tags, and more.              //
//////////////////////////////////////////////////////////////////////////

pub struct Actor {
    location: Location,
    meta_tags: Vec<HashMap<String, String>>,
}

pub struct Planet {
    actor_data: Actor,
    object_file: Vec<Region>,
}

 
///////////////////////////////// Example planet ////////////////////////////////
//  fn main() {
//      let myplanet = Planet {
//          actor_data: Actor {
//              location: Location {
//                  rotation: Rotation { w: 1.0, x: 0.0, y: 0.0, z: 0.0 },
//                  scale3D: Scale { x: 1.0, y: 1.0, z: 1.0 },
//                  translation: Translation { x: 0.0, y: 0.0, z: 0.0 },
//              },
//              meta_tags: vec![
//                  {
//                      let mut map = HashMap::new();
//                      map.insert(String::from("tag1"), String::from("value1"));
//                      map
//                  },
//                  {
//                      let mut map = HashMap::new();
//                      map.insert(String::from("tag2"), String::from("value2"));
//                      map
//                  },
//              ],
//          },
//          contained_region: vec![
//              Region {
//                  location: (0,0),
//                  chunks: vec![
//                      Chunk {
//                          id: 1,
//                          data: vec![0, 1, 2, 3],
//                      },
//                      Chunk {
//                          id: 2,
//                          data: vec![4, 5, 6, 7],
//                      },
//                  ],
//              },
//              Region {
//                  location: (0, 0),
//                  chunks: vec![
//                      Chunk {
//                          id: 3,
//                          data: vec![8, 9, 10, 11],
//                      },
//                  ],
//              },
//          ],
//      };
//  }
//////////////////////////////////////////////////////////////////////////////////