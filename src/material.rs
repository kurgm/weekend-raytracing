use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ir: f64 },
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
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
                let scattered = Ray::new(
                    hit_record.p,
                    reflected + *fuzz * Vec3::random_in_unit_sphere(),
                );
                let attenuation = *albedo;
                if scattered.direction.dot(&hit_record.normal) > 0.0 {
                    Some((scattered, attenuation))
                } else {
                    None
                }
            }
            Material::Dielectric { ir } => {
                let refraction_ratio = if hit_record.front_face {
                    1.0 / *ir
                } else {
                    *ir
                };

                let unit_direction = ray_in.direction.unit();

                let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
                let reflectance = dielectric_reflectance(cos_theta, refraction_ratio);

                let refracted = (reflectance <= rand::random::<f64>())
                    .then(|| unit_direction.refract(&hit_record.normal, refraction_ratio))
                    .flatten();
                let direction =
                    refracted.unwrap_or_else(|| unit_direction.reflect(&hit_record.normal));

                let scattered = Ray::new(hit_record.p, direction);
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                Some((scattered, attenuation))
            }
        }
    }
}

fn dielectric_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
