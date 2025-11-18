use std::io::prelude::*;

use super::Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Vec3) {
	let r = (255.999 * pixel_color.x()) as i32;
	let g = (255.999 * pixel_color.y()) as i32;
	let b = (255.999 * pixel_color.z()) as i32;

	writeln!(out, "{r} {g} {b}").expect("writing color");
}
