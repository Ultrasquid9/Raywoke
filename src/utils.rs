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
	/// fast inverse square root constant.
	const MAGIC: u64 = 0x5FE6EB50C7B537A9;

	/// Calculates the square root. uses x86-specific methods when possible, otherwise 
	/// falls back to [fast inverse square root](en.wikipedia.org/wiki/Fast_inverse_square_root).
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

	/// Calculates the square root using the 
	/// [fast inverse square root](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
	#[allow(unused)]
	fn sqrt_magic(input: f64) -> f64 {
		let num = MAGIC - (input.to_bits() >> 1);
		let mut num = 1. / f64::from_bits(num);

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
		assert_eq!(4., sqrt_magic(16.).round());
	}

	#[test]
	fn less_simple() {
		assert_eq!(346., (sqrt(12.) * 100.).round());
		assert_eq!(346., (sqrt_magic(12.) * 100.).round());
	}
}
