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

#[derive(Debug, Clone, Copy)]
pub enum MaterialValue {
	Metal(Metal),
	Lambertian(Lambertian),
	Dielectric(Dielectric),
}

impl Material for MaterialValue {
	fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
		match self {
			Self::Metal(m) => m.scatter(r_in, rec),
			Self::Lambertian(l) => l.scatter(r_in, rec),
			Self::Dielectric(d) => d.scatter(r_in, rec),
		}
	}
}

impl From<Metal> for MaterialValue {
	fn from(value: Metal) -> Self {
		Self::Metal(value)
	}
}

impl From<Lambertian> for MaterialValue {
	fn from(value: Lambertian) -> Self {
		Self::Lambertian(value)
	}
}

impl From<Dielectric> for MaterialValue {
	fn from(value: Dielectric) -> Self {
		Self::Dielectric(value)
	}
}
