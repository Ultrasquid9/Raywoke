#![cfg(feature = "yakui")]

use super::Point;

impl Point for yakui::geometry::Vec2 {
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
