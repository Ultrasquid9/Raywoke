#![cfg(feature = "mint")]

use crate::prelude::*;
use mint::*;

point! { Vector2<f32>, |x, y| Vector2 { x: x as f32, y: y as f32 }}
point! { Vector2<f64>, |x, y| Vector2 { x, y }}
point! { Point2<f32>, |x, y| Point2 { x: x as f32, y: y as f32 }}
point! { Point2<f64>, |x, y| Point2 { x, y }}
