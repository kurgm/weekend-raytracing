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

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

fn pixel_color(i: u32, j: u32, world: &impl Hittable, camera: &Camera) -> Vec3 {
    let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
    let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    let r = camera.get_ray(u, v);
    ray_color(&r, world)
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Vec3 {
    if let Some(hit) = world.hit(ray, (Bound::Included(0.0), Bound::Unbounded)) {
        let n = hit.normal.unit();
        return 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
