#![cfg(feature = "ultraviolet")]

use ultraviolet::{DVec2, Vec2};

use super::Point;

impl Point for Vec2 {
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

impl Point for DVec2 {
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
