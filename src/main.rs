use std::io;

use ray_tracer::{
	Camera, HitRecord, Hittable, HittableList, Ray, Sphere, Vec3, color, utils, vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

fn main() {
	let mut world = HittableList::default();
	world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
	world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

	let cam = Camera::new();

	println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

	for j in (0..IMAGE_HEIGHT).rev() {
		eprint!("\rScanlines remaining: {j:0>3}");
		for i in 0..IMAGE_WIDTH {
			let mut pixel_color = Vec3::default();
			for _ in 0..SAMPLES_PER_PIXEL {
				let u = (f64::from(i) + utils::random_double()) / f64::from(IMAGE_WIDTH - 1);
				let v = (f64::from(j) + utils::random_double()) / f64::from(IMAGE_HEIGHT - 1);
				let r = cam.get_ray(u, v);
				pixel_color += ray_color(r, &world);
			}

			color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
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
