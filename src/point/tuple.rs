use super::Point;

impl Point for (f32, f32) {
	fn x(&self) -> f64 {
		self.0 as f64
	}

	fn y(&self) -> f64 {
		self.1 as f64
	}

	fn edit(&mut self, x: f64, y: f64) {
		self.0 = x as f32;
		self.1 = y as f32;
	}
}

impl Point for (f64, f64) {
	fn x(&self) -> f64 {
		self.0
	}

	fn y(&self) -> f64 {
		self.1
	}

	fn edit(&mut self, x: f64, y: f64) {
		self.0 = x;
		self.1 = y;
	}
}
