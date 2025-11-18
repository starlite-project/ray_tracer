use std::{
	io::{Result as IoResult, prelude::*},
	simd::{StdFloat as _, prelude::*},
};

use super::Vec3;

pub fn write_color<W: Write>(
	out: &mut W,
	pixel_color: Vec3,
	samples_per_pixel: i32,
) -> IoResult<()> {
	let simd = pixel_color.to_simd();

	let scale = f64x4::splat(1.0 / f64::from(samples_per_pixel));

	let result = (scale * simd).sqrt();
	let clamped = result.simd_clamp(f64x4::splat(0.0), f64x4::splat(0.999));

	let [r, g, b, _] = clamped.to_array();

	writeln!(
		out,
		"{} {} {}",
		(256.0 * r) as i32,
		(256.0 * g) as i32,
		(256.0 * b) as i32,
	)
}
