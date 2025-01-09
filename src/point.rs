use dyn_clone::DynClone;

pub mod glam;
pub mod nalgebra;
pub mod tuple;
pub mod yakui;

/// A point that can be used for Raycasting.
/** # Examples
```
#[derive(Debug, Clone)]
struct Vec2 {
	x: f32,
	y: f32
}

impl Point for Vec2 {
	fn x(&self) -> f32 {
		self.x
	}
	fn y(&self) -> f32 {
		self.y
	}
	fn edit(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}
}
```
 */
pub trait Point: DynClone + core::fmt::Debug + Send + Sync {
	fn x(&self) -> f32;
	fn y(&self) -> f32;

	fn edit(&mut self, x: f32, y: f32);
}

dyn_clone::clone_trait_object!(Point);
