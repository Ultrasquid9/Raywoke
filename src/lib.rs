#![allow(clippy::tabs_in_doc_comments)]
/*!
# Raywoke

Raywoke is an extremely simple raycasting crate, forked from [raylite](https://github.com/heyimrein/raylite). It was created primarily to make the API simpler to use, and integrate more closely with third-party math libraries.

## Third-party crate interop

Raywoke provides interop with the following external crates:
- `cgmath`
- `euclid`
- `glam`
- `mint`
- `nalgebra`
- `ultraviolet`

To enable this, enable their respective features in your `cargo.toml`:

```toml
[dependencies]
raywoke = { version = "0.1", features = ["glam","nalgebra"] }
```

## Examples

**Using the library**
```rust
use raywoke::prelude::*;

// Tuples are being used here for demonstration purposes, but any type which implements the Point trait will work
let ray = Ray::new(
	(0., 0.),
	(2., 0.),
);

let mut bar = Barrier::new(
	(1., -1.),
	(1., 1.)
);

let result = cast(&ray, &bar); // Returns a Result<RayHit, RayFail>

assert!(result.is_ok()); // Result is an Ok<RayHit> containing hit info

// Place barrier behind the Ray
bar = Barrier::new(
	(-1., -1.),
	(-1., 1.)
);

let result = cast(&ray, &bar);
assert!(result.is_err()); // Result is an Err<RayFail::NoHit>
```

**Third-party crate interop**
```rust
use glam::DVec2;
use nalgebra::Vector2;
use raywoke::prelude::*;

// With the "glam" and "nalgebra" features, you can use their respective Vector structs
let ray = Ray::new(
	DVec2::new(0., 0.),
	Vector2::new(0., 0.),
);
```

**Creating your own Point struct**
```rust
use raywoke::prelude::*;

struct Vec2 {
	x: f64,
	y: f64
}

// The "point!" macro derives the trait automatically
// You can also implement it manually
point! { Vec2, f64 }

let ray = Ray::new(
	Vec2 { x: 0., y: 0. },
	Vec2 { x: 2., y: 0. },
);
```
 */

#![cfg_attr(feature = "no_std", no_std)]

use prelude::*;
use utils::distance;

pub mod hit_fail;
pub mod point;
pub mod prelude;
mod utils;

/// Cast a ray for collision detection, with only the consideration of a single [Barrier].
pub fn cast(ray: &Ray, bar: &Barrier) -> Result<RayHit, RayFail> {
	let den = (ray.start.x() - ray.end.x()) * (bar.0.y() - bar.1.y())
		- (ray.start.y() - ray.end.y()) * (bar.0.x() - bar.1.x());
	if den == 0. {
		return Err(RayFail::Parallel);
	}

	let t_num = (ray.start.x() - bar.0.x()) * (bar.0.y() - bar.1.y())
		- (ray.start.y() - bar.0.y()) * (bar.0.x() - bar.1.x());
	let u_num = (ray.start.x() - bar.0.x()) * (ray.start.y() - ray.end.y())
		- (ray.start.y() - bar.0.y()) * (ray.start.x() - ray.end.x());

	let range = 0. ..=1.;
	let t = t_num / den;
	let u = u_num / den;

	if range.contains(&t) && range.contains(&u) {
		let mut point = ray.start;
		point.edit(
			ray.start.x() + t * (ray.end.x() - ray.start.x()),
			ray.start.y() + t * (ray.end.y() - ray.start.y()),
		);

		return Ok(RayHit {
			position: point,
			distance: distance(ray.start, point),
		});
	}
	Err(RayFail::NoHit)
}

/// Cast a Ray for collision detection, with the consideration of several [Barrier]'s.
///
/// `bars` should have at least 1 element.
///
/// The (possibly) returned hit information will represent the closest barrier to `ray`'s
/// origin point that was hit.
pub fn cast_wide(ray: &Ray, bars: &[Barrier]) -> Result<RayHit, RayFail> {
	if bars.is_empty() {
		return Err(RayFail::NoBars);
	}

	let mut ray_clone = ray.clone();
	let mut result = Err(RayFail::NoHit);

	for bar in bars {
		match cast(&ray_clone, bar) {
			Ok(hit) => {
				ray_clone.end = hit.position;
				result = Ok(hit);
			}
			Err(e) => {
				if result.is_err() {
					result = Err(e);
				}
			}
		}
	}

	result
}

/// Raycast collision unit, the basis for all raycast collision detection.
/// Determines the conditions under which collision will be detected.
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
	/// Origin position the Ray will emit from.
	pub start: (f64, f64),
	/// Position representing the end of the Ray.
	pub end: (f64, f64),
}

/// 1-dimensional collision subject; Solid line.
/// Simplest building block for collider objects.
#[derive(Debug, Clone, PartialEq)]
pub struct Barrier(pub (f64, f64), pub (f64, f64));

impl Ray {
	pub fn new(start: impl Point, end: impl Point) -> Self {
		Self {
			start: start.tup_f64(),
			end: end.tup_f64(),
		}
	}
}

impl Barrier {
	pub fn new(start: impl Point, end: impl Point) -> Self {
		Self(start.tup_f64(), end.tup_f64())
	}
}
