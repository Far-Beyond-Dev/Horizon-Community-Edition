use spade::{DelaunayTriangulation, Point2, Triangulation};

pub fn perform_triangulation(points: Vec<(f64, f64, f64)>) {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for point in points {
        triangulation.insert(Point2::new(point.0, point.1)).expect("Insertion failed");
    }

    // Iterate over all vertices and print their positions
//    println!("Vertices:");
//    for vertex in triangulation.vertices() {
//        let position = vertex.position();
//        println!("Poits.Add(FVector2D({}, {}));", position.x, position.y);
//    }

    // Iterate over all undirected edges and print the positions of the vertices they connect
//    println!("Edges:");
//    for edge in triangulation.undirected_edges() {
//        let vertices = edge.vertices();
//        let from = vertices[0];
//        let to = vertices[1];
//        println!(
//            "Edges.Add(FEdge(FVector2D({}, {}), FVector2D({}, {})));",
//            from.position().x, from.position().y,
//            to.position().x, to.position().y
//        );
//    }
}
