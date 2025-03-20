#![cfg(feature = "nalgebra")]

use super::Point;

impl Point for nalgebra::Vector2<f32> {
	fn x(&self) -> f64 {
		self.x as f64
	}

	fn y(&self) -> f64 {
		self.y as f64
	}

	fn edit(&mut self, x: f64, y: f64) {
		self.x = x as f64;
		self.y = y as f64;
	}
}

impl Point for nalgebra::Vector2<f64> {
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
