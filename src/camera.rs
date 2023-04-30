use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let viewport_height: f64 = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width: f64 = aspect_ratio * viewport_height;
        const FOCAL_LENGTH: f64 = 1.0;

        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - FOCAL_LENGTH * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let screen_point = self.lower_left_corner + s * self.horizontal + t * self.vertical;
        Ray::new(self.origin, screen_point - self.origin)
    }
}
