use spade::{DelaunayTriangulation, Point2, Triangulation};
use spade::handles::VoronoiVertex;
pub fn perform_triangulation(points: Vec<(f64, f64, f64)>) -> DelaunayTriangulation<Point2<f64>> {
    let mut triangulation = DelaunayTriangulation::new();

    for point in points {
        // Project the 3D point onto a 2D plane using spherical coordinates
        let (x, y, z) = point;
        let lat = (z / 1000.0).asin();
        let lon = y.atan2(x);
        triangulation.insert(Point2::new(lon, lat)).expect("Insertion failed");
    }

    triangulation
}
pub fn generate_voronoi(triangulation: &DelaunayTriangulation<Point2<f64>>) {
    println!("Voronoi Edges:");
    for edge in triangulation.undirected_voronoi_edges() {
        match edge.vertices() {
            [VoronoiVertex::Inner(from), VoronoiVertex::Inner(to)] => {
                let from_center = from.circumcenter();
                let to_center = to.circumcenter();
                println!(
                    "DrawDebugLine(GetWorld(), FVector({:.4}, {:.4}, 0), FVector({:.4}, {:.4}, 0), FColor::Red, true, -1.0f, 0, 2.0f);",
                    from_center.x, from_center.y, to_center.x, to_center.y
                );
            }
            [VoronoiVertex::Inner(from), VoronoiVertex::Outer(edge)] | 
            [VoronoiVertex::Outer(edge), VoronoiVertex::Inner(from)] => {
                let from_center = from.circumcenter();
                let direction = edge.direction_vector();
                let scale = 1000.0; // Adjust this value as needed
                println!(
                    "DrawDebugLine(GetWorld(), FVector({:.4}, {:.4}, 0), FVector({:.4}, {:.4}, 0), FColor::Blue, true, -1.0f, 0, 2.0f);",
                    from_center.x, from_center.y, 
                    from_center.x + direction.x * scale, from_center.y + direction.y * scale
                );
            }
            [VoronoiVertex::Outer(_), VoronoiVertex::Outer(_)] => {
                // This case only happens if all vertices of the triangulation lie on the
                // same line and can probably be ignored.
            }
        }
    }
}

fn project_to_sphere(point: &Point2<f64>, radius: f64) -> (f64, f64, f64) {
    let x = point.x;
    let y = point.y;
    let z = (radius * radius - x * x - y * y).sqrt();
    (x, y, z)
}

fn spherical_to_cartesian(lon: f64, lat: f64, radius: f64) -> (f64, f64, f64) {
    let x = radius * lat.cos() * lon.cos();
    let y = radius * lat.cos() * lon.sin();
    let z = radius * lat.sin();
    (x, y, z)
}

pub fn generate_voronoi_on_sphere(triangulation: &DelaunayTriangulation<Point2<f64>>, radius: f64) {
    println!("Voronoi Edges on Sphere:");
    for edge in triangulation.undirected_voronoi_edges() {
        match edge.vertices() {
            [VoronoiVertex::Inner(from), VoronoiVertex::Inner(to)] => {
                let from_center = from.circumcenter();
                let to_center = to.circumcenter();
                let from_3d = spherical_to_cartesian(from_center.x, from_center.y, radius);
                let to_3d = spherical_to_cartesian(to_center.x, to_center.y, radius);
                println!(
                    "DrawDebugLine(GetWorld(), FVector({:.4}, {:.4}, {:.4}), FVector({:.4}, {:.4}, {:.4}), FColor::Red, true, -1.0f, 0, 2.0f);",
                    from_3d.0, from_3d.1, from_3d.2, to_3d.0, to_3d.1, to_3d.2
                );
            }
            [VoronoiVertex::Inner(from), VoronoiVertex::Outer(edge)] | 
            [VoronoiVertex::Outer(edge), VoronoiVertex::Inner(from)] => {
                let from_center = from.circumcenter();
                let direction = edge.direction_vector();
                let scale = 0.1; // Adjust this value as needed
                let from_3d = spherical_to_cartesian(from_center.x, from_center.y, radius);
                let to_lon = from_center.x + direction.x * scale;
                let to_lat = from_center.y + direction.y * scale;
                let to_3d = spherical_to_cartesian(to_lon, to_lat, radius);
                println!(
                    "DrawDebugLine(GetWorld(), FVector({:.4}, {:.4}, {:.4}), FVector({:.4}, {:.4}, {:.4}), FColor::Blue, true, -1.0f, 0, 2.0f);",
                    from_3d.0, from_3d.1, from_3d.2, to_3d.0, to_3d.1, to_3d.2
                );
            }
            [VoronoiVertex::Outer(_), VoronoiVertex::Outer(_)] => {
                // This case only happens if all vertices of the triangulation lie on the
                // same line and can probably be ignored.
            }
        }
    }
}