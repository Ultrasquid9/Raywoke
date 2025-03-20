pub mod glam;
pub mod nalgebra;
pub mod tuple;
pub mod yakui;

/// A point that can be used for Raycasting.
/** # Examples
```
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
