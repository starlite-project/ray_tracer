use crate::{
	HitRecord, Material, Ray, ScatterRecord,
	vec3::{self, Vec3},
};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Lambertian {
	albedo: Vec3,
}

impl Lambertian {
	#[must_use]
	pub const fn new(albedo: Vec3) -> Self {
		Self { albedo }
	}
}

impl Material for Lambertian {
	fn scatter(&self, _: Ray, rec: &HitRecord) -> Option<ScatterRecord> {
		let mut scatter_direction = rec.normal + vec3::random_unit_vector();

		if scatter_direction.is_near_zero() {
			scatter_direction = rec.normal;
		}

		Some(ScatterRecord {
			attenuation: self.albedo,
			scattered: Ray::new(rec.p, scatter_direction),
		})
	}
}
