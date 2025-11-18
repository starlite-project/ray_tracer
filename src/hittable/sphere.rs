use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::Arc,
};

use crate::{
	HitRecord, Hittable, Material, Ray,
	vec3::{self, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
	center: Vec3,
	radius: f64,
	mat: Arc<dyn Material>,
}

impl Sphere {
	#[must_use]
	pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
		Self {
			center,
			radius,
			mat,
		}
	}
}

impl Debug for Sphere {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Sphere")
			.field("center", &self.center)
			.field("radius", &self.radius)
			.finish_non_exhaustive()
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = vec3::dot(oc, r.direction());
		let c = self.radius.mul_add(-self.radius, oc.length_squared());
		let discriminant = half_b.mul_add(half_b, -(a * c));
		if discriminant < 0.0 {
			return false;
		}

		let sqrt_d = discriminant.sqrt();

		let mut root = (-half_b - sqrt_d) / a;
		if root <= t_min || t_max <= root {
			root = (-half_b + sqrt_d) / a;
			if root <= t_min || t_max <= root {
				return false;
			}
		}

		rec.t = root;
		rec.p = r.at(rec.t);
		let outward_normal = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);
		rec.mat = Some(self.mat.clone());
		true
	}
}
