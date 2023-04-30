use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
}

impl Material {
    pub fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                let scattered = Ray::new(hit_record.p, scatter_direction);
                let attenuation = *albedo;
                Some((scattered, attenuation))
            }
        }
    }
}
