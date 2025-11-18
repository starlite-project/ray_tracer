const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;
fn main() {

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let r = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let g = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\nDone.");
}
