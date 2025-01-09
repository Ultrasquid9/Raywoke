#![cfg(feature = "yakui")]

use super::Point;

impl Point for yakui::geometry::Vec2 {
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
