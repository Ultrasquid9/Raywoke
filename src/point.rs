use dyn_clone::DynClone;

pub mod glam;
pub mod nalgebra;
pub mod tuple;

pub trait Point: DynClone + core::fmt::Debug {
	fn x(&self) -> f32;
	fn y(&self) -> f32;

	fn edit(&mut self, x: f32, y: f32);
}

dyn_clone::clone_trait_object!(Point);
