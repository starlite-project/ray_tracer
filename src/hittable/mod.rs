mod list;
mod sphere;

use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	rc::Rc,
};

pub use self::{list::*, sphere::*};
use super::{
	Material, Ray,
	vec3::{self, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub mat: Option<Rc<dyn Material>>,
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

pub trait Hittable {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
