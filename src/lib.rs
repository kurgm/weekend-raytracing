use rand::Rng;
use std::{error::Error, ops::Bound};

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

mod camera;
mod hittable;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

pub fn run() -> Result<(), Box<dyn Error>> {
    let camera = Camera::new();

    let world: HittableList = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let Vec3 { x: r, y: g, z: b } = pixel_color(i, j, &world, &camera);

            // Gamma correction
            let r = r.sqrt();
            let g = g.sqrt();
            let b = b.sqrt();

            let ir = (256.0 * r.clamp(0.0, 0.999)) as u32;
            let ig = (256.0 * g.clamp(0.0, 0.999)) as u32;
            let ib = (256.0 * b.clamp(0.0, 0.999)) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn pixel_color(i: u32, j: u32, world: &impl Hittable, camera: &Camera) -> Vec3 {
    let mut rng = rand::thread_rng();
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    (0..SAMPLES_PER_PIXEL)
        .map(|_| {
            let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let r = camera.get_ray(u, v);

            ray_color(&r, world, MAX_DEPTH)
        })
        .sum::<Vec3>()
        / SAMPLES_PER_PIXEL as f64
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, (Bound::Excluded(0.001), Bound::Unbounded)) {
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);
    }
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
