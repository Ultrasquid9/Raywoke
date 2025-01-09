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
