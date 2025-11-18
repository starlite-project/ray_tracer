use std::io;

use ray_tracer::{HitRecord, Hittable, HittableList, Ray, Sphere, Vec3, color, utils, vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
	let viewport_height = 2.0;
	let viewport_width = ASPECT_RATIO * viewport_height;
	let focal_length = 1.0;

	let origin = Vec3::default();
	let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
	let vertical = Vec3::new(0.0, viewport_height, 0.0);
	let lower_left_corner =
		origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

	let mut world = HittableList::default();
	world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
	world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

	println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

	for j in (0..IMAGE_HEIGHT).rev() {
		eprint!("\rScanlines remaining: {j}");
		for i in 0..IMAGE_WIDTH {
			let u = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
			let v = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
			let r = Ray::new(
				origin,
				lower_left_corner + u * horizontal + v * vertical - origin,
			);
			let pixel_color = ray_color(r, &world);
			color::write_color(&mut io::stdout(), pixel_color);
		}
	}

	eprintln!("\nDone.");
}

fn ray_color(r: Ray, world: &dyn Hittable) -> Vec3 {
	let mut rec = HitRecord::default();
	if world.hit(r, 0.0, utils::INFINITY, &mut rec) {
		return 0.5 * (rec.normal + Vec3::splat(1.0));
	}

	let unit_direction = vec3::unit_vector(r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Vec3::splat(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
