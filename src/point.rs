pub mod cgmath;
pub mod euclid;
pub mod glam;
pub mod mint;
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
}
```
 */
pub trait Point: Send + Sync {
	fn x(&self) -> f64;
	fn y(&self) -> f64;

	fn tup_f32(&self) -> (f32, f32) {
		(self.x() as f32, self.y() as f32)
	}
	fn tup_f64(&self) -> (f64, f64) {
		(self.x(), self.y())
	}
}

/// Implements the point trait on the given type.
///
/// Accepts the type for the trait to be implemented on, and the type of the "x" and "y" fields.
/// If the struct lacks an "x" or a "y" field, those should also be specified.
///
/// For more complex scenarios (for example, x and y having different types),
/// then the trait should instead be implemented manually.
/** # Examples
```
use raywoke::prelude::*;

// If the struct has an "x" and a "y" field,
// then they will be used automatically
struct Vec2 {
	x: f64,
	y: f64,
}
point! { Vec2, f64 }

// If the struct lacks an "x" or a "y" field,
// then the intended fields should be specified
struct Position {
	pos_x: f32,
	pos_y: f32,
}
point! { Position, f32, pos_x, pos_y }
```
 */
#[macro_export]
macro_rules! point {
	($point:ty, $type:ty, $x:tt, $y:tt) => {
		impl $crate::point::Point for $point {
			fn x(&self) -> f64 {
				self.$x as f64
			}

			fn y(&self) -> f64 {
				self.$y as f64
			}
		}
	};
	($point:ty, $type:ty) => {
		$crate::point::point! { $point, $type, x, y }
	};
}
pub use point;
