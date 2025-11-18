use crate::{HitRecord, Hittable, Ray};

#[derive(Default)]
#[repr(transparent)]
pub struct HittableList {
	objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
	pub fn add<H>(&mut self, object: H)
	where
		H: Hittable + 'static,
	{
		self.objects.push(Box::new(object));
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut temp_rec = None;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			if let Some(rec) = object.hit(r, t_min, closest_so_far) {
				closest_so_far = rec.t;
				temp_rec = Some(rec);
			}
		}

		temp_rec
	}
}
