mod dielectric;
mod lamb;
mod metal;

pub use self::{dielectric::*, lamb::*, metal::*};
use super::{HitRecord, Ray, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct ScatterRecord {
	pub attenuation: Vec3,
	pub scattered: Ray,
}

pub trait Material: Send + Sync {
	fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
