use crate::P;

/// Calculate the distance between two 2D points.
pub fn distance(p1: P, p2: P) -> f32 {
	return ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt();
}
