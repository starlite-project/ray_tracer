use std::{
	fmt::{Display, Formatter, Result as FmtResult, Write as _},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::utils;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
}

impl Vec3 {
	#[must_use]
	pub const fn new(x: f64, y: f64, z: f64) -> Self {
		Self { x, y, z }
	}

	#[must_use]
	pub const fn splat(i: f64) -> Self {
		Self::new(i, i, i)
	}

	#[must_use]
	pub const fn x(self) -> f64 {
		self.x
	}

	#[must_use]
	pub const fn y(self) -> f64 {
		self.y
	}

	#[must_use]
	pub const fn z(self) -> f64 {
		self.z
	}

	#[must_use]
	pub fn length(self) -> f64 {
		self.length_squared().sqrt()
	}

	#[must_use]
	pub fn length_squared(self) -> f64 {
		self.z
			.mul_add(self.z, self.x.mul_add(self.x, self.y * self.y))
	}

	#[must_use]
	pub fn random() -> Self {
		Self::new(
			utils::random_double(),
			utils::random_double(),
			utils::random_double(),
		)
	}

	#[must_use]
	pub fn random_range(min: f64, max: f64) -> Self {
		Self::new(
			utils::random_double_range(min, max),
			utils::random_double_range(min, max),
			utils::random_double_range(min, max),
		)
	}

	#[must_use]
	pub const fn is_near_zero(self) -> bool {
		self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
	}
}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl Add<f64> for Vec3 {
	type Output = Self;

	fn add(self, rhs: f64) -> Self::Output {
		Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Display for Vec3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.x, f)?;
		f.write_char(' ')?;
		Display::fmt(&self.y, f)?;
		f.write_char(' ')?;
		Display::fmt(&self.z, f)
	}
}

impl Div for Vec3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
	}
}

impl Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output {
		self / Self::splat(rhs)
	}
}

impl DivAssign for Vec3 {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		*self = *self / rhs;
	}
}

impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
	}
}

impl Mul<f64> for Vec3 {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		self * Self::splat(rhs)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Vec3::splat(self) * rhs
	}
}

impl MulAssign for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		*self = *self * rhs;
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::new(-self.x, -self.y, -self.z)
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

#[must_use]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
	u.z.mul_add(v.z, u.x.mul_add(v.x, u.y * v.y))
}

#[must_use]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
	Vec3::new(
		u.y.mul_add(v.z, -(u.z * v.y)),
		u.z.mul_add(v.x, -(u.x * v.z)),
		u.x.mul_add(v.y, -(u.y * v.x)),
	)
}

#[must_use]
pub fn unit_vector(v: Vec3) -> Vec3 {
	v / v.length()
}

#[must_use]
pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_range(-1.0, 1.0);
		if p.length_squared() >= 1.0 {
			continue;
		}

		break p;
	}
}

#[must_use]
pub fn random_unit_vector() -> Vec3 {
	unit_vector(random_in_unit_sphere())
}

#[must_use]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	v - 2.0 * dot(v, n) * n
}

#[must_use]
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = f64::min(dot(-uv, n), 1.0);
	let r_out_perp = etai_over_etat * (uv + cos_theta * n);
	let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
	r_out_perp + r_out_parallel
}

#[must_use]
pub fn random_in_unit_disk() -> Vec3 {
	loop {
		let p = Vec3::new(
			utils::random_double_range(-1.0, 1.0),
			utils::random_double_range(-1.0, 1.0),
			0.0,
		);

		if p.length_squared() >= 1.0 {
			continue;
		}

		break p;
	}
}
