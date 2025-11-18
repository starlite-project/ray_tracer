pub use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;

#[must_use]
pub const fn degrees_to_radians(degrees: f64) -> f64 {
	degrees.to_radians()
}

#[must_use]
pub fn random_double() -> f64 {
	// rand::rng().random()
	fastrand::f64()
}

#[must_use]
pub fn random_double_range(min: f64, max: f64) -> f64 {
	(max - min).mul_add(random_double(), min)
}

#[must_use]
pub const fn clamp(x: f64, min: f64, max: f64) -> f64 {
	f64::clamp(x, min, max)
}
