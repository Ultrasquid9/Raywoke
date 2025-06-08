#![cfg(feature = "cgmath")]

use crate::prelude::*;
use cgmath::*;

point! { Vector2<f32>, |x, y| Vector2::new(x as f32, y as f32) }
point! { Vector2<f64>, Vector2::new }
point! { Point2<f32>, |x, y| Point2::new(x as f32, y as f32) }
point! { Point2<f64>, Point2::new }
