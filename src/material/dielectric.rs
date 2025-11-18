use crate::{HitRecord, Material, Ray, Vec3, utils, vec3};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Dielectric {
	ir: f64,
}

impl Dielectric {
	#[must_use]
	pub const fn new(ir: f64) -> Self {
		Self { ir }
	}

	fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		(1.0 - r0).mul_add(f64::powf(1.0 - cosine, 5.0), r0)
	}
}

impl Material for Dielectric {
	fn scatter(
		&self,
		r_in: Ray,
		rec: &HitRecord,
		attenuation: &mut Vec3,
		scattered: &mut Ray,
	) -> bool {
		let refraction_ratio = if rec.front_face {
			1.0 / self.ir
		} else {
			self.ir
		};

		let unit_direction = vec3::unit_vector(r_in.direction());
		let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract
			|| Self::reflectance(cos_theta, refraction_ratio) > utils::random_double()
		{
			vec3::reflect(unit_direction, rec.normal)
		} else {
			vec3::refract(unit_direction, rec.normal, refraction_ratio)
		};

		*attenuation = Vec3::splat(1.0);
		*scattered = Ray::new(rec.p, direction);
		true
	}
}
