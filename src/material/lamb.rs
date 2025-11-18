use crate::{
	HitRecord, Material, Ray,
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
	fn scatter(
		&self,
		_: Ray,
		rec: &HitRecord,
		attenuation: &mut Vec3,
		scattered: &mut Ray,
	) -> bool {
		let mut scatter_direction = rec.normal + vec3::random_unit_vector();

		if scatter_direction.is_near_zero() {
			scatter_direction = rec.normal;
		}

		*attenuation = self.albedo;
		*scattered = Ray::new(rec.p, scatter_direction);
		true
	}
}
