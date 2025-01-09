/*!
# Raywoke

Raywoke is an extremely simple raycasting crate, forked from [raylite](https://github.com/heyimrein/raylite). It was created primarily to make the API simpler to use, and integrate more closely with third-party math libraries.

In order to achieve this, Raywoke makes two compromises:
- It is no longer `no_std`
- It now requires `dyn-clone`, meaning it is no longer dependency-free. 

## Glam and Nalgebra interop

Raywoke provides interop with both Glam and Nalgebra. To enable this, enable their restpective features in your `cargo.toml`:

```toml
[dependencies]
raywoke = { version = "0.", features = ["glam","nalgebra"] }
```

## Examples

**Using the library**
```rust
use raywoke::prelude::*;

fn main() {
    // Positions are differentiated here because emission direction matters
    let ray = Ray {
        start: (0., 0.), // Emission origin position
        end: (2., 0.),   // Emission end position
    };
	// Direction does not matter for Barriers
    let mut bar = Barrier ((1., -1.), (1., 1.)); 

    let result = cast(&ray, &bar); // Returns a Result<RayHit, RayFail>

    assert!(result.is_ok()); // Result is an Ok<RayHit> containing hit info

    bar = Barrier ((-1., -1.), (-1., 1.)); // Place barrier behind the Ray

    let result = cast(&ray, &bar);
    assert!(result.is_err()); // Result is an Err<RayFail::NoHit>
}
```

**Glam and Nalgebra interop**
```rust
use glam::Vec2;
use nalgebra::Vector2;
use raywoke::prelude::*;

fn main() {
	// With the "glam" and "nalgebra" features, you can use their respective Vector structs
	let ray = Ray::new(
		Vec2::new(0., 0.),
		Vector2::new(0., 0.),
	);
}
```

**Creating your own Point struct**
```rust
use raywoke::prelude::*;

// Clone and Debug are both required
#[derive(Debug, Clone)]
struct Vec2 {
	x: f32,
	y: f32
}

impl Point for Vec2 {
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

fn main() {
	let ray = Ray::new(
		Vec2 { x: 0., y: 0. },
		Vec2 { x: 2., y: 0. },
	);
}
```
 */

use point::Point;
use utils::distance;

pub mod point;
pub mod prelude;
mod utils;

pub(crate) type P = Box<dyn Point>;

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

	let t = t_num / den;
	let u = u_num / den;

	if (t >= 0. && t <= 1.) && (u >= 0. && u <= 1.) {
		let mut point = ray.start.clone();
		point.edit(
			ray.start.x() + t * (ray.end.x() - ray.start.x()), 
			ray.start.y() + t * (ray.end.y() - ray.start.y())
		);

		return Ok(RayHit {
			position: point.clone(),
			distance: distance(ray.start.clone(), point.clone()),
		});
	}
	return Err(RayFail::NoHit);
	}

/// Cast a Ray for collision detection, with the consideration of several [Barrier]'s.
///
/// `bars` must have at least 1 element.
///
/// The (possibly) returned hit information will represent the closest barrier to `ray`'s
/// origin point that was hit.
pub fn cast_wide(ray: &Ray, bars: &[Barrier]) -> Result<RayHit, RayFail> {
	if bars.len() <= 0 {
		panic!("Barrier vector cannot be empty!");
	}

	let mut ray_clone = ray.clone();

	let mut hit: Option<RayHit> = None;
	for bar in bars {
		match cast(&ray_clone, bar) {
			Ok(v) => {
				ray_clone.end = v.position.clone();
				hit = Some(v);
			}
			Err(_) => continue,
		}
	}

	if hit.is_some() {
		return Ok(hit.unwrap());
	}

	return Err(RayFail::NoHit);
}

/// Raycast failure states.
pub enum RayFail {
	/// Did not hit any colliders.
	///
	/// *Universal*
	NoHit,
	/// Ray and Barrier are parallel; cannot collide.
	///
	/// *Exclusive to isolated checks* -> [cast]
	Parallel,
}

/// Raycast collision data.
pub struct RayHit {
	/// Position of collision point.
	pub position: P,
	/// Distance of collision point from Ray emission origin.
	pub distance: f32,
}

/// Raycast collision unit, the basis for all raycast collision detection.
/// Determines the conditions under which collision will be detected.
#[derive(Debug, Clone)]
pub struct Ray {
	/// Origin position the Ray will emit from.
	pub start: P,
	/// Position representing the end of the Ray.
	pub end: P,
}

/// 1-dimensional collision subject; Solid line.
/// Simplest building block for collider objects.
#[derive(Debug, Clone)]
pub struct Barrier (pub P, pub P);

impl Ray {
	pub fn new(start: impl Point + 'static, end: impl Point + 'static) -> Self {
		Self {
			start: Box::new(start),
			end: Box::new(end)
		}
	}
}

impl Barrier {
	pub fn new(start: impl Point + 'static, end: impl Point + 'static) -> Self {
		Self (
			Box::new(start),
			Box::new(end)
		)
	}
}
