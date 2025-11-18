use std::io::prelude::*;

use super::{Vec3, utils};

pub fn write_color<W: Write>(out: &mut W, pixel_color: Vec3, samples_per_pixel: i32) {
	let mut r = pixel_color.x();
	let mut g = pixel_color.y();
	let mut b = pixel_color.z();

	let scale = 1.0 / f64::from(samples_per_pixel);

	r *= scale;
	g *= scale;
	b *= scale;

	writeln!(
		out,
		"{} {} {}",
		(256.0 * utils::clamp(r, 0.0, 0.999)) as i32,
		(256.0 * utils::clamp(g, 0.0, 0.999)) as i32,
		(256.0 * utils::clamp(b, 0.0, 0.999))
	)
	.expect("writing color");
}
