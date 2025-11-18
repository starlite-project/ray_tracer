use std::{io, rc::Rc};

use ray_tracer::{
	Camera, Dielectric, HitRecord, Hittable, HittableList, Lambertian, Metal, Ray, Sphere, Vec3,
	color, utils, vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
	let mut world = HittableList::default();

	let ground_material = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
	let center_material = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
	let left_material = Rc::new(Dielectric::new(1.5));
	let right_material = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

	world.add(Sphere::new(
		Vec3::new(0.0, -100.5, -1.0),
		100.0,
		ground_material,
	));
	world.add(Sphere::new(
		Vec3::new(-1.0, 0.0, -1.0),
		-0.4,
		left_material.clone(),
	));
	world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, center_material));
	world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left_material));
	world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right_material));

	let cam = Camera::new();

	println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

	let mut stdout = io::stdout().lock();
	for j in (0..IMAGE_HEIGHT).rev() {
		eprint!("\rScanlines remaining: {j:0>3}");
		for i in 0..IMAGE_WIDTH {
			let mut pixel_color = Vec3::default();
			for _ in 0..SAMPLES_PER_PIXEL {
				let u = (f64::from(i) + utils::random_double()) / f64::from(IMAGE_WIDTH - 1);
				let v = (f64::from(j) + utils::random_double()) / f64::from(IMAGE_HEIGHT - 1);
				let r = cam.get_ray(u, v);
				pixel_color += ray_color(r, &world, MAX_DEPTH);
			}

			color::write_color(&mut stdout, pixel_color, SAMPLES_PER_PIXEL);
		}
	}

	eprintln!("\nDone.");
}

fn ray_color<H: Hittable>(r: Ray, world: &H, depth: i32) -> Vec3 {
	if depth <= 0 {
		return Vec3::default();
	}

	let mut rec = HitRecord::default();
	if world.hit(r, 0.001, utils::INFINITY, &mut rec) {
		let mut attenuation = Vec3::default();
		let mut scattered = Ray::default();

		if rec
			.mat
			.as_ref()
			.unwrap()
			.scatter(r, &rec, &mut attenuation, &mut scattered)
		{
			return attenuation * ray_color(scattered, world, depth - 1);
		}

		return Vec3::default();
	}

	let unit_direction = vec3::unit_vector(r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Vec3::splat(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
