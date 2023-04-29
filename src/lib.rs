use std::error::Error;

use crate::vec3::Vec3;

mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let Vec3 { x: r, y: g, z: b } = calculate_color(i, j);

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn calculate_color(i: u32, j: u32) -> Vec3 {
    let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
    let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    let b = 0.25f64;
    Vec3 { x: r, y: g, z: b }
}
