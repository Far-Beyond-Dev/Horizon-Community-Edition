use rusqlite::{params, Connection, Result as SqlResult};
use serde_json::{self, Value};
use serde::{Serialize, Deserialize};
use std::fs;
use uuid::Uuid;  // Make sure to add the uuid crate to your dependencies

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    id: Option<Uuid>,
    x: f64,
    y: f64,
    z: f64,
    data: Value,
}

pub struct Database {
    conn: Connection,
}

impl Point {
    pub fn new(id: Option<Uuid>, x: f64, y: f64, z: f64, data: Value) -> Self {
        Point { id, x, y, z, data }
    }
}

impl Database {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn create_table(&self) -> SqlResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS points (
                id TEXT PRIMARY KEY,
                x REAL NOT NULL,
                y REAL NOT NULL,
                z REAL NOT NULL,
                dataFile TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Adds a new point to the database and writes its data to a file.
    ///
    /// This function performs the following operations:
    /// 1. Serializes the point's data to a JSON string.
    /// 2. Generates a UUID for the point if not provided.
    /// 3. Creates a file path based on the UUID.
    /// 4. Writes the point's data to a file at the generated path.
    /// 5. Inserts the point's information into the database.
    ///
    /// # Arguments
    ///
    /// * `point` - A reference to the `Point` struct containing the point's data.
    ///
    /// # Returns
    ///
    /// * `SqlResult<()>` - Ok(()) if the operation was successful, or an error if it failed.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The point's data cannot be serialized to JSON.
    /// * The file cannot be written to disk.
    /// * The database insertion fails.
    ///
    /// # File Storage Structure
    ///
    /// Files are stored in a two-level directory structure:
    /// * The base directory is `./data/`
    /// * The first level subdirectory is named with the first two characters of the UUID.
    /// * The file is named with the full UUID.
    ///
    /// This structure helps prevent filesystem overload and improves search performance.
    pub fn add_point(&self, point: &Point) -> SqlResult<()> {
        let data_str = serde_json::to_string(&point.data)
            .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
        
        let id = point.id.unwrap_or_else(Uuid::new_v4).to_string();

        ////////////////////////////////////////////////////////////////////
        // Generate the paths and names we will later use to              //
        // store data that is oversized or does not fit the table schema  //
        ////////////////////////////////////////////////////////////////////

        // We pass a UUID to this function that is created from the seed system,
        // in this way we have a deterministic identifier for the objects

        let contents = point.data.to_string();
        let UUIDUnwrap = point.id.unwrap();
        let UUIDString: String = UUIDUnwrap.to_string();

        // The first two characters of the UUID become a folder name, this is to
        // help prevent the UNIX fs from being overloaded with the large number of
        // small files and to make searching faster

        let folderName: String = UUIDString.chars().take(2).collect();
        let filePath: String = "./data/".to_string() + &folderName.to_string() + &UUIDString;

        // Now we write the actual file to the disk at the correct location

        let Result: Result<(), std::io::Error> = fs::write(&filePath, contents);
        ////////////////////////////////////////
        // TODO: This print causes an error!  //
        // println!(Result.to_string());      //
        ////////////////////////////////////////

        // Next if the write was successful we will add a matching entry to the database
        // if not we will throw an error and crash

        self.conn.execute(
            "INSERT INTO points (id, x, y, z, data) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, point.x, point.y, point.z, &filePath],
        )?;
        
        Ok(())
    }

    pub fn get_points_within_radius(&self, x1: f64, y1: f64, z1: f64, radius: f64) -> SqlResult<Vec<Point>> {
        let radius_sq = radius * radius;
        let mut stmt = self.conn.prepare(
            "SELECT id, x, y, z, data FROM points
             WHERE ((x - ?1) * (x - ?1) + (y - ?2) * (y - ?2) + (z - ?3) * (z - ?3)) <= ?4",
        )?;
        
        let points_iter = stmt.query_map(params![x1, y1, z1, radius_sq], |row| {
            let id: String = row.get(0)?;
            let x: f64 = row.get(1)?;
            let y: f64 = row.get(2)?;
            let z: f64 = row.get(3)?;
            let data_str: String = row.get(4)?;
            let data: Value = serde_json::from_str(&data_str).unwrap();
            Ok(Point {
                id: Some(Uuid::parse_str(&id).unwrap()),
                x,
                y,
                z,
                data,
            })
        })?;
        
        let mut points = Vec::new();
        for point in points_iter {
            points.push(point?);
        }
        
        Ok(points)
    }
}