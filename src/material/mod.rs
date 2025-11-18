mod dielectric;
mod lamb;
mod metal;

pub use self::{dielectric::*, lamb::*, metal::*};
use super::{HitRecord, Ray, Vec3};

pub trait Material: Send + Sync {
	fn scatter(
		&self,
		r_in: Ray,
		rec: &HitRecord,
		attenuation: &mut Vec3,
		scattered: &mut Ray,
	) -> bool;
}
