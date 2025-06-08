#![cfg(feature = "euclid")]

// Euclid is wierd
macro_rules! euclid_pointify {
	($point:ty, $num:ty) => {
		impl<U: Send + Sync> $crate::point::Point for $point {
			fn x(&self) -> f64 {
				self.x as f64
			}

			fn y(&self) -> f64 {
				self.y as f64
			}
		}
	};
}

euclid_pointify! { euclid::Point2D<f32, U>, f32 }
euclid_pointify! { euclid::Point2D<f64, U>, f64 }

euclid_pointify! { euclid::Vector2D<f32, U>, f32 }
euclid_pointify! { euclid::Vector2D<f64, U>, f64 }
