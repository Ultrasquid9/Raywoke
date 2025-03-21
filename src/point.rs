pub mod cgmath;
pub mod glam;
pub mod nalgebra;
pub mod tuple;
pub mod ultraviolet;

/// A point that can be used for Raycasting.
/** # Examples
```
use raywoke::prelude::*;

#[derive(Debug, Clone)]
struct Vec2 {
	x: f64,
	y: f64
}

impl Point for Vec2 {
	fn x(&self) -> f64 {
		self.x
	}
	fn y(&self) -> f64 {
		self.y
	}
	fn edit(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
	}
}
```
 */
pub trait Point: core::fmt::Debug + Send + Sync {
	fn x(&self) -> f64;
	fn y(&self) -> f64;
	fn edit(&mut self, x: f64, y: f64);

	fn tup_f32(&self) -> (f32, f32) {
		(self.x() as f32, self.y() as f32)
	}
	fn tup_f64(&self) -> (f64, f64) {
		(self.x(), self.y())
	}
}

// Macros to automatically implement the Point trait for a given type
#[allow(unused)]
macro_rules! pointify_f32 {
	($point:ty) => {
		impl $crate::point::Point for $point {
			fn x(&self) -> f64 {
				self.x as f64
			}

			fn y(&self) -> f64 {
				self.y as f64
			}

			fn edit(&mut self, x: f64, y: f64) {
				self.x = x as f32;
				self.y = y as f32;
			}
		}
	};
}
#[allow(unused)]
macro_rules! pointify_f64 {
	($point:ty) => {
		impl $crate::point::Point for $point {
			fn x(&self) -> f64 {
				self.x
			}

			fn y(&self) -> f64 {
				self.y
			}

			fn edit(&mut self, x: f64, y: f64) {
				self.x = x;
				self.y = y;
			}
		}
	};
}
#[allow(unused)]
pub(crate) use pointify_f32;
#[allow(unused)]
pub(crate) use pointify_f64;
