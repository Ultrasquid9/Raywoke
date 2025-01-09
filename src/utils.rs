use crate::P;

/// Calculate the distance between two 2D points.
pub fn distance(p1: P, p2: P) -> f32 {
	#[cfg(feature = "no_std")]
	return sqrt(pow2(p2.x() - p1.x()) + pow2(p2.y() - p1.y()));

	#[cfg(not(feature = "no_std"))]
	return ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt();
}

#[cfg(feature = "no_std")]
fn sqrt(input: f32) -> f32 {
	let mut i = u32::from_le_bytes(input.to_le_bytes());

	i = 0x1fbd3f7d + (i >> 1);

	let mut num = f32::from_le_bytes(i.to_le_bytes());

	for _ in 0..2 {
		num = (num + (input / num)) / 2.;
	}

	num
}

#[cfg(feature = "no_std")]
fn pow2(input: f32) -> f32 {
	input * input
}
