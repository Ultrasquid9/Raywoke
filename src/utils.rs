use crate::prelude::*;
#[cfg(feature = "no_std")]
use no_std::*;

/// Calculate the distance between two 2D points.
pub fn distance(p1: impl Point, p2: impl Point) -> f64 {
	#[cfg(feature = "no_std")]
	return sqrt(pow2(p2.x() - p1.x()) + pow2(p2.y() - p1.y()));

	#[cfg(not(feature = "no_std"))]
	return ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt();
}

#[cfg(feature = "no_std")]
mod no_std {
	/// Calculates the square root. This algorithm is based upon [this blogpost](https://suraj.sh/fast-square-root-approximation).
	pub fn sqrt(input: f64) -> f64 {
		let input = input as f32;

		let mut i = u32::from_le_bytes(input.to_le_bytes());
		i = 0x1fbd3f7d + (i >> 1);
		let mut num = f32::from_le_bytes(i.to_le_bytes());

		num = (num + (input / num)) / 2.;
		num = (num + (input / num)) / 2.;

		num as f64
	}

	pub fn pow2(input: f64) -> f64 {
		input * input
	}

	#[test]
	fn simple() {
		assert_eq!(4., sqrt(16.));
	}

	#[test]
	fn less_simple() {
		assert_eq!(346., (sqrt(12.) * 100.).round());
	}
}
