# Raywoke

Raywoke is an extremely simple raycasting crate, forked from [raylite](https://github.com/heyimrein/raylite). It was created primarily to make the API simpler to use, and integrate more closely with third-party math libraries.

## Third-party crate interop

Raywoke provides interop with the following external crates:
- `cgmath`
- `glam`
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

// Clone and Debug are both required
#[derive(Debug, Clone)]
struct Vec2 {
	x: f64,
	y: f64
}

impl Point for Vec2 {
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

let ray = Ray::new(
	Vec2 { x: 0., y: 0. },
	Vec2 { x: 2., y: 0. },
);
```
