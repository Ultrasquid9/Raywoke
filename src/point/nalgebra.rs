#![cfg(feature = "nalgebra")]

use nalgebra::Vector2;
use crate::prelude::*;

point! { Vector2<f32>, |x, y| Vector2::new(x as f32, y as f32) }
point! { Vector2<f64>, Vector2::new }
