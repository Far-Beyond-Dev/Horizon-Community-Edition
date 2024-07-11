mod fibonacci_sphere;
mod delaunay_triangulation;
mod space;

use fibonacci_sphere::generate_fibonacci_sphere;
use delaunay_triangulation::{perform_triangulation, generate_voronoi_on_sphere};

pub fn main() {
    println!("Generating Sphere...");
    let num_samples = 1000;
    let radius = 1000.0;

    let points = generate_fibonacci_sphere(num_samples, radius);
    let triangulation = perform_triangulation(points);

    // Generate Voronoi diagram projected onto a full sphere
    generate_voronoi_on_sphere(&triangulation, radius);

    println!("Done");

    space::simulate();
}
