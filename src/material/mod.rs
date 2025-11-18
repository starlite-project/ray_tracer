mod lamb;
mod metal;

pub use self::{lamb::*, metal::*};
use super::{HitRecord, Ray, Vec3};

pub trait Material {
	fn scatter(
		&self,
		r_in: Ray,
		rec: &HitRecord,
		attenuation: &mut Vec3,
		scattered: &mut Ray,
	) -> bool;
}
