#![feature(portable_simd)]

use std::{
	io::{self, Result as IoResult, prelude::*},
	simd::prelude::*,
	sync::Arc,
};

use ray_tracer::{
	Camera, Dielectric, Hittable, HittableList, Lambertian, Metal, Ray, Sphere, Vec3, color, utils,
	vec3,
};
use rayon::prelude::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;
const IMAGE_BOUND: f64x2 = f64x2::from_array([(IMAGE_WIDTH - 1) as f64, (IMAGE_HEIGHT - 1) as f64]);

fn main() -> IoResult<()> {
	let world = random_scene();

	let lookfrom = Vec3::new(13.0, 2.0, 3.0);
	let lookat = Vec3::new(0.0, 0.0, -1.0);
	let vup = Vec3::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.1;

	let cam = Camera::new(
		lookfrom,
		lookat,
		vup,
		20.0,
		ASPECT_RATIO,
		aperture,
		dist_to_focus,
	);

	println!("P6\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

	let mut out = Vec::with_capacity(0x0170_0000);

	for j in (0..IMAGE_HEIGHT).rev() {
		eprint!("\rScanlines remaining: {j:0>3}");
		let pixel_colors = (0..IMAGE_WIDTH)
			.into_par_iter()
			.map(|i| {
				let mut pixel_color = Vec3::default();
				for _ in 0..SAMPLES_PER_PIXEL {
					let u = (f64::from(i) + utils::random_double()) / f64::from(IMAGE_WIDTH - 1);
					let v = (f64::from(j) + utils::random_double()) / f64::from(IMAGE_HEIGHT - 1);
					let r = cam.get_ray(u, v);
					pixel_color += ray_color(r, &world, MAX_DEPTH);
				}

				pixel_color
			})
			.collect::<Vec<_>>();

		for pixel_color in pixel_colors {
			color::write_color(&mut out, pixel_color, SAMPLES_PER_PIXEL)?;
		}
	}

	io::stdout().write_all(&out)?;

	eprintln!("\nDone. Output length = {}.", out.len());
	Ok(())
}

fn ray_color<H: Hittable>(r: Ray, world: &H, depth: i32) -> Vec3 {
	if depth <= 0 {
		return Vec3::default();
	}

	if let Some(rec) = world.hit(r, 0.001, utils::INFINITY) {
		if let Some(scatter) = rec.mat.scatter(r, &rec) {
			return scatter.attenuation * ray_color(scatter.scattered, world, depth - 1);
		}

		return Vec3::default();
	}

	let unit_direction = vec3::unit_vector(r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Vec3::splat(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
	let mut world = HittableList::default();

	let ground_material = Arc::new(Lambertian::new(Vec3::splat(0.5)));
	world.add(Sphere::new(
		Vec3::new(0.0, -1000.0, 0.0),
		1000.0,
		ground_material,
	));

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = utils::random_double();
			let center = Vec3::new(
				0.9f64.mul_add(utils::random_double(), f64::from(a)),
				0.2,
				0.9f64.mul_add(utils::random_double(), f64::from(b)),
			);

			if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
				if choose_mat < 0.8 {
					let albedo = Vec3::random() * Vec3::random();
					let sphere_material = Arc::new(Lambertian::new(albedo));
					world.add(Sphere::new(center, 0.2, sphere_material));
				} else if choose_mat < 0.95 {
					let albedo = Vec3::random_range(0.5, 1.0);
					let fuzz = utils::random_double_range(0.0, 0.5);
					let sphere_material = Arc::new(Metal::new(albedo, fuzz));
					world.add(Sphere::new(center, 0.2, sphere_material));
				} else {
					let sphere_material = Arc::new(Dielectric::new(1.5));
					world.add(Sphere::new(center, 0.2, sphere_material));
				}
			}
		}
	}

	let material_1 = Arc::new(Dielectric::new(1.5));
	world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1));

	let material_2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
	world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2));

	let material_3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
	world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3));

	world
}
