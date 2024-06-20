use spade::{DelaunayTriangulation, Triangulation, Point2, InsertionError};

pub fn perform_triangulation(points: Vec<(f64, f64, f64)>) -> Result<DelaunayTriangulation<Point2<f64>>, InsertionError> {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for point in points {
        triangulation.insert(Point2::new(point.0, point.1))?;
    }

    // add the z value to the triangulation

    Ok(triangulation)
}
