use rand::{distributions::Distribution, Rng};
use rayon::prelude::*;
use std::{error::Error, ops::Bound};

use camera::Camera;
use hittable::{Hittable, Sphere};
use material::Material;
use ray::Ray;
use vec3::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 3.0 / 2.0;

const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

pub fn run() -> Result<(), Box<dyn Error>> {
    let camera = {
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, 0.1, 10.0)
    };

    let world = random_scene();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut image: Vec<_> = (0..IMAGE_HEIGHT)
        .rev()
        .flat_map(|j| {
            eprint!("\rScanlines remaining: {} ", j);
            (0..IMAGE_WIDTH).map(move |i| (i, j))
        })
        .par_bridge()
        .map(|(i, j)| ((IMAGE_HEIGHT - j, i), pixel_color(i, j, &world, &camera)))
        .collect();

    image.sort_unstable_by_key(|(pos, _)| *pos);

    for (_, color) in image {
        let Vec3 { x: r, y: g, z: b } = color;

        // Gamma correction
        let r = r.sqrt();
        let g = g.sqrt();
        let b = b.sqrt();

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u32;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u32;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u32;

        println!("{} {} {}", ir, ig, ib);
    }
    eprintln!("\nDone.");
    Ok(())
}

fn pixel_color(i: u32, j: u32, world: &Hittable, camera: &Camera) -> Vec3 {
    let mut rng = rand::thread_rng();
    const SAMPLES_PER_PIXEL: i32 = 500;
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

fn ray_color(ray: &Ray, world: &Hittable, depth: i32) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, (Bound::Excluded(0.001), Bound::Unbounded)) {
        let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) else {
            return Vec3::new(0.0, 0.0, 0.0);
        };
        return attenuation * ray_color(&scattered, world, depth - 1);
    }
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> Hittable {
    let mut world = vec![
        Hittable::Sphere(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            },
        )),
        Hittable::Sphere(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Material::Dielectric { ir: 1.5 },
        )),
        Hittable::Sphere(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambertian {
                albedo: Vec3::new(0.4, 0.2, 0.1),
            },
        )),
        Hittable::Sphere(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            },
        )),
    ];

    let dist = rand::distributions::WeightedIndex::new([0.8, 0.15, 0.05]).unwrap();
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }
            let material = match dist.sample(&mut rng) {
                0 => {
                    // diffuse
                    let albedo = Vec3::random(-1.0..1.0) * Vec3::random(-1.0..1.0);
                    Material::Lambertian { albedo }
                }
                1 => {
                    // metal
                    let albedo = Vec3::random(0.5..1.0);
                    let fuzz = rng.gen::<f64>() * 0.5;
                    Material::Metal { albedo, fuzz }
                }
                2 => {
                    // glass
                    Material::Dielectric { ir: 1.5 }
                }
                _ => unreachable!(),
            };
            world.push(Hittable::Sphere(Sphere::new(center, 0.2, material)));
        }
    }

    Hittable::HittableList(world)
}
