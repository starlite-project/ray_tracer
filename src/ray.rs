use super::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
	origin: Vec3,
	direction: Vec3,
}

impl Ray {
	#[must_use]
	pub const fn new(origin: Vec3, direction: Vec3) -> Self {
		Self { origin, direction }
	}

	#[must_use]
	pub const fn origin(self) -> Vec3 {
		self.origin
	}

	#[must_use]
	pub const fn direction(self) -> Vec3 {
		self.direction
	}

	#[must_use]
	pub fn at(self, t: f64) -> Vec3 {
		self.origin + t * self.direction
	}
}
