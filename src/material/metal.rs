use crate::{
	HitRecord, Material, Ray,
	vec3::{self, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
	albedo: Vec3,
	fuzz: f64,
}

impl Metal {
	#[must_use]
	pub const fn new(albedo: Vec3, fuzz: f64) -> Self {
		Self { albedo, fuzz }
	}
}

impl Material for Metal {
	fn scatter(
		&self,
		r_in: Ray,
		rec: &HitRecord,
		attenuation: &mut Vec3,
		scattered: &mut Ray,
	) -> bool {
		let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);

		*attenuation = self.albedo;
		*scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
		vec3::dot(scattered.direction(), rec.normal) > 0.0
	}
}
