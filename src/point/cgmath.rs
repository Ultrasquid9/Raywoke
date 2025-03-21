#![cfg(feature = "cgmath")]

use cgmath::{Point2, Vector2};

use super::Point;

impl Point for Vector2<f32> {
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

impl Point for Vector2<f64> {
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

impl Point for Point2<f32> {
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

impl Point for Point2<f64> {
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
