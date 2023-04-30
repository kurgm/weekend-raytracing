use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = {
                    let dir = hit_record.normal + Vec3::random_unit_vector();
                    // Catch degenerate scatter direction
                    if dir.near_zero() {
                        hit_record.normal
                    } else {
                        dir
                    }
                };
                let scattered = Ray::new(hit_record.p, scatter_direction);
                let attenuation = *albedo;
                Some((scattered, attenuation))
            }
            Material::Metal { albedo } => {
                let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
                let scattered = Ray::new(hit_record.p, reflected);
                let attenuation = *albedo;
                if scattered.direction.dot(&hit_record.normal) > 0.0 {
                    Some((scattered, attenuation))
                } else {
                    None
                }
            }
        }
    }
}
