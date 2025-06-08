use crate::prelude::*;

impl Point for (f32, f32) {
	xy!(0, 1);

	fn from_point(other: &impl Point) -> Self {
		(other.x() as f32, other.y() as f32)
	}
}

impl Point for (f64, f64) {
	xy!(0, 1);

	fn from_point(other: &impl Point) -> Self {
		(other.x(), other.y())
	}	
}
