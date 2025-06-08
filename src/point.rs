pub mod cgmath;
pub mod euclid;
pub mod glam;
pub mod mint;
pub mod nalgebra;
pub mod tuple;
pub mod ultraviolet;

/// A point that can be used for Raycasting.
/// # Examples
///```
/// use raywoke::prelude::*;
/// 
/// #[derive(Debug, Clone)]
/// struct Vec2 {
/// 	x: f64,
/// 	y: f64
/// }
/// 
/// impl Point for Vec2 {
/// 	fn x(&self) -> f64 {
/// 		self.x
/// 	}
/// 
/// 	fn y(&self) -> f64 {
/// 		self.y
/// 	}
/// 
/// 	fn from_point(other: &impl Point) -> Self {
/// 		Self {
/// 			x: other.x(), 
/// 			y: other.y(),
/// 		}
/// 	}
/// }
///```
pub trait Point: Send + Sync {
	fn x(&self) -> f64;
	fn y(&self) -> f64;

	fn from_point(other: &impl Point) -> Self;

	fn to_point<P: Point>(&self) -> P 
	where 
		Self: Sized
	{
		P::from_point(self)
	}
}

/// Specify the `x` and `y` fields of a [Point].
/// 
/// For more complex scenarios (for example, `x` or `y` not implementing `Into<f64>`),
/// then they can instead be specified manually.
/// # Examples
/// ```
/// use raywoke::prelude::*;
/// 
/// struct Vec2 {
/// 	x: f64,
/// 	y: f64,
/// }
/// 
/// impl Point for Vec2 {
/// 	xy! { x, y }
/// 
/// 	# fn from_point(other: &impl Point) -> Self { stringify!(
/// 	fn from_point(other: &impl Point) -> Self {...}
/// 	# );Self{x:other.x(),y:other.y()}}
/// }
/// ```
#[macro_export]
macro_rules! xy {
	($x:tt, $y:tt) => {
		fn x(&self) -> f64 {
			self.$x.into()
		}

		fn y(&self) -> f64 {
			self.$y.into()
		}
	};
}

/// Implement the [Point] trait on a given type.
/// 
/// If the type does not have fields named `x` or `y`, you should
/// implement the trait manually and use the [xy] macro instead.
/// # Examples
/// ```
/// use raywoke::prelude::*;
/// 
/// struct Vec2 {
/// 	x: f64,
/// 	y: f64,
/// }
/// 
/// impl Vec2 {
/// 	# pub fn new(x:f64,y:f64)->Self{stringify!(
/// 	pub fn new(x: f64, y: f64) -> Self {...}
/// 	# );Self{x,y}}
/// }
/// 
/// point! { Vec2, Vec2::new }
/// ```
#[macro_export]
macro_rules! point {
	($point:ty, $constructor:expr) => {
		impl $crate::point::Point for $point {
			$crate::point::xy!{ x, y }

			fn from_point(other: &impl Point) -> Self {
				$constructor(other.x(), other.y())
			}
		}
	};
}

pub use xy;
pub use point;
