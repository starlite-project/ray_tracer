mod list;
mod sphere;

use std::fmt::{Debug, Formatter, Result as FmtResult};

pub use self::{list::*, sphere::*};
use super::{
	MaterialValue, Ray,
	vec3::{self, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub mat: MaterialValue,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
		self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
		self.normal = if self.front_face {
			outward_normal
		} else {
			-outward_normal
		};
	}
}

impl Debug for HitRecord {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("HitRecord")
			.field("p", &self.p)
			.field("normal", &self.normal)
			.field("t", &self.t)
			.field("front_face", &self.front_face)
			.finish_non_exhaustive()
	}
}

pub trait Hittable: Send + Sync {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub enum HittableValue {
	Sphere(Sphere),
	List(HittableList),
}

impl Hittable for HittableValue {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		match self {
			Self::Sphere(s) => s.hit(r, t_min, t_max),
			Self::List(l) => l.hit(r, t_min, t_max),
		}
	}
}

impl From<Sphere> for HittableValue {
	fn from(value: Sphere) -> Self {
		Self::Sphere(value)
	}
}

impl From<HittableList> for HittableValue {
	fn from(value: HittableList) -> Self {
		Self::List(value)
	}
}
