use std::{error::Error, ops::Bound};

use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

mod hittable;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

pub fn run() -> Result<(), Box<dyn Error>> {
    let world: HittableList = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let Vec3 { x: r, y: g, z: b } = pixel_color(i, j, &world);

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
    Ok(())
}

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn pixel_color(i: u32, j: u32, world: &impl Hittable) -> Vec3 {
    const ORIGIN: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    const HORIZONTAL: Vec3 = Vec3 {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };
    const VERTICAL: Vec3 = Vec3 {
        x: 0.0,
        y: VIEWPORT_HEIGHT,
        z: 0.0,
    };

    let lower_left_corner: Vec3 = ORIGIN
        - HORIZONTAL / 2.0
        - VERTICAL / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        };

    let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
    let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    let r = Ray::new(
        ORIGIN,
        lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
    );
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
