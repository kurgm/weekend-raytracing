use std::ops::{Bound, RangeBounds};

use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn from_outward_normal_and_ray(ray: &Ray, t: f64, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::new(ray.at(t), normal, t, front_face)
    }
}

pub type RangeF64 = (Bound<f64>, Bound<f64>);

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: RangeF64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: RangeF64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = 'a: {
            let root = (-half_b - sqrtd) / a;
            if t_range.contains(&root) {
                break 'a Some(root);
            }
            let root = (-half_b + sqrtd) / a;
            if t_range.contains(&root) {
                break 'a Some(root);
            }
            None
        };
        root.map(|t| {
            let p = ray.at(t);
            let outward_normal = (p - self.center) / self.radius;
            HitRecord::from_outward_normal_and_ray(ray, t, outward_normal)
        })
    }
}
