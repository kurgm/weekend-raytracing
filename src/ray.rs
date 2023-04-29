use crate::{
    hittable::{Hittable, Sphere},
    Vec3,
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Vec3 {
        let t = self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
        let unit = self.direction.unit();
        let t = 0.5 * (unit.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        Sphere::new(center, radius)
            .hit(self, 0.0..f64::INFINITY)
            .map(|r| r.t)
            .unwrap_or(-1.0)
    }
}
