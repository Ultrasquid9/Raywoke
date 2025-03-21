use core::{
	error::Error,
	fmt::{self, Debug, Display, write},
};

/// Raycast failure states.
#[derive(Debug, Clone, PartialEq)]
pub enum RayFail {
	/// Ray and Barrier are parallel; cannot collide.
	///
	/// *Universal*
	Parallel,
	/// Did not hit any colliders.
	///
	/// *Universal*
	NoHit,
	/// No Barrier was provided; nothing to collide with.
	///
	/// *Exclusive to [cast_wide](super::cast_wide)
	NoBars,
}

/// Raycast collision data.
#[derive(Debug, Clone, PartialEq)]
pub struct RayHit {
	/// Position of collision point.
	pub position: (f64, f64),
	/// Distance of collision point from Ray emission origin.
	pub distance: f64,
}

impl Display for RayFail {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			RayFail::Parallel => write(f, format_args!("Parallel")),
			RayFail::NoHit => write(f, format_args!("NoHit")),
			RayFail::NoBars => write(f, format_args!("NoBars")),
		}
	}
}

impl Error for RayFail {}
