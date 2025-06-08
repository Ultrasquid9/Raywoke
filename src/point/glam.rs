#![cfg(feature = "glam")]

use crate::prelude::*;
use glam::*;

point! { Vec2, |x, y| Vec2::new(x as f32, y as f32)}
point! { DVec2, DVec2::new}
