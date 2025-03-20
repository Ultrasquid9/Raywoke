use core::{
	error::Error,
	fmt::{self, Debug, Display, write},
};

/// Raycast failure states.
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
pub struct RayHit {
	/// Position of collision point.
	pub position: (f64, f64),
	/// Distance of collision point from Ray emission origin.
	pub distance: f64,
}

impl RayFail {
	fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = |str: &str| write(f, format_args!("{str}"));
		match self {
			RayFail::Parallel => output("Parallel"),
			RayFail::NoHit => output("NoHit"),
			RayFail::NoBars => output("NoBars"),
		}
	}
}

impl Display for RayFail {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.display(f)
	}
}

impl Debug for RayFail {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.display(f)
	}
}

impl Error for RayFail {}
