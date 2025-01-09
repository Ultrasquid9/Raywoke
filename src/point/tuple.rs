use super::Point;

impl Point for (f32, f32) {
	fn x(&self) -> f32 {
		self.0
	}

	fn y(&self) -> f32 {
		self.1
	}

	fn edit(&mut self, x: f32, y: f32) {
		self.0 = x;
		self.1 = y;
	}
}

impl Point for (f64, f64) {
	fn x(&self) -> f32 {
		self.0 as f32
	}

	fn y(&self) -> f32 {
		self.1 as f32
	}

	fn edit(&mut self, x: f32, y: f32) {
		self.0 = x as f64;
		self.1 = y as f64;
	}
}
