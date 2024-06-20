mod fibonacci_sphere;
mod delaunay_triangulation;

use fibonacci_sphere::generate_fibonacci_sphere;
use delaunay_triangulation::perform_triangulation;


fn main() {
    let num_samples = 1000;
    let min_latitude = -90.0;
    let max_latitude = 90.0;
    let min_longitude = -180.0;
    let max_longitude = 180.0;
    let seed = 0.1;

    match generate_fibonacci_sphere(num_samples, min_latitude, max_latitude, min_longitude, max_longitude, seed) {
        Ok(points) => {
            println!("Generated {} points", points.len());
            let triangulation = perform_triangulation(points);
            println!("Triangulation contains {} vertices and {} faces", triangulation.num_vertices(), triangulation.num_faces());
            
            // If you need to process the triangulation further, you can do it here.
            for vertex in triangulation.vertices() {
                let x = vertex.position().x;
                let y = vertex.position().y;

                // Find the corresponding z value from the original points
                if let Some(&(_, _, z)) = points.iter().find(|&&(px, py, _)| (px - x).abs() < f64::EPSILON && (py - y).abs() < f64::EPSILON) {
                    println!("Vertex: ({}, {}, {})", x, y, z);
                }
            }
        },
        Err(e) => {
            eprintln!("Error generating points: {}", e);
        }
    }
}
