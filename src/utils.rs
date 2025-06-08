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
	/// fast sqrt magic number, taken from [this blogpost](https://suraj.sh/fast-square-root-approximation).
	const MAGIC: u32 = 0x1FBD3F7D;

	/// Calculates the square root. uses x86-specific methods when possible, otherwise 
	/// falls back to [fast square root](en.wikipedia.org/wiki/Fast_inverse_square_root).
	pub fn sqrt(input: f64) -> f64 {
		#[cfg(all(
			any(target_arch = "x86", target_arch = "x86_64"),
			target_feature = "sse2"
		))]
		return sqrt_arch(input);

		#[cfg(not(all(
			any(target_arch = "x86", target_arch = "x86_64"),
			target_feature = "sse2"
		)))]
		return sqrt_magic(input);
	}

	/// Calculates the square root using the fast inverse sqrt method.
	#[allow(unused)]
	fn sqrt_magic(input: f64) -> f64 {
		let input = input as f32;

		let num = MAGIC + (input.to_bits() >> 1);
		let mut num = f32::from_bits(num);

		num = (num + (input / num)) / 2.;
		num = (num + (input / num)) / 2.;

		num as f64
	}

	/// Calculates the square root using x86 exclusive methods.
	#[cfg(all(
		any(target_arch = "x86", target_arch = "x86_64"),
		target_feature = "sse2"
	))]
	fn sqrt_arch(input: f64) -> f64 {
		#[cfg(target_arch = "x86_64")]
		use core::arch::x86_64::*;
		#[cfg(target_arch = "x86")]
		use core::arch::x86::*;

		// SAFETY: This function can only be called if on x86
		// and if the sse2 feature is enabled. 
		unsafe {
			let i = _mm_set1_pd(input);
			let sqrt = _mm_sqrt_pd(i);
			_mm_cvtsd_f64(sqrt)
		}
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
