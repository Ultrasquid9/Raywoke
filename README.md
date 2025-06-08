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
raywoke = { version = "0.2", features = ["glam","nalgebra"] }
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

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {...}
}

// The "point!" macro derives the trait automatically
// You can also implement it manually, if needed
point! { Vec2, Vec2::new }

let ray = Ray::new(
	Vec2::new(0., 0.),
	Vec2::new(2., 0.),
);
```
