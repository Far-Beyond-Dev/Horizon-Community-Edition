use spade::{DelaunayTriangulation, Triangulation, Point2, InsertionError};

pub fn perform_triangulation(points: Vec<(f64, f64, f64)>) -> Result<DelaunayTriangulation<Point2<f64>>, spade::InsertionError> {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for point in points {
        triangulation.insert(Point2::new(point.0, point.1))?;
    }

    Ok(triangulation)
}
