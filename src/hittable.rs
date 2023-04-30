use std::ops::{Bound, RangeBounds};

use crate::{material::Material, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool, material: Material) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn from_outward_normal_and_ray(
        ray: &Ray,
        t: f64,
        outward_normal: Vec3,
        material: Material,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::new(ray.at(t), normal, t, front_face, material)
    }
}

pub type RangeF64 = (Bound<f64>, Bound<f64>);

#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(Sphere),
    HittableList(Vec<Hittable>),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_range: RangeF64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_range),
            Hittable::HittableList(list) => hit_list(list, ray, t_range),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

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
            HitRecord::from_outward_normal_and_ray(ray, t, outward_normal, self.material)
        })
    }
}

fn hit_list(list: &Vec<Hittable>, ray: &Ray, t_range: RangeF64) -> Option<HitRecord> {
    let mut closest_so_far = t_range.1;
    let mut hit_record = None;
    for object in list {
        if let Some(record) = object.hit(ray, (t_range.0, closest_so_far)) {
            closest_so_far = Bound::Excluded(record.t);
            hit_record = Some(record);
        }
    }
    hit_record
}
