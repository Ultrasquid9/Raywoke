#![cfg(feature = "nalgebra")]

use super::Point;

impl Point for nalgebra::Vector2<f32> {
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

impl Point for nalgebra::Vector2<f64> {
	fn x(&self) -> f32 {
		self.x as f32
	}

	fn y(&self) -> f32 {
		self.y as f32
	}

	fn edit(&mut self, x: f32, y: f32) {
		self.x = x as f64;
		self.y = y as f64;
	}
}
