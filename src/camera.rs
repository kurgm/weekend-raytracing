use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let viewport_height: f64 = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width: f64 = aspect_ratio * viewport_height;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner: Vec3 =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let screen_point = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(self.origin, screen_point - self.origin)
    }
}
