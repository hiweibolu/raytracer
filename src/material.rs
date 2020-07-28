pub use crate::hit::HitResult;
pub use crate::random::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitResult) -> Option<(Vec3, Ray)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzzy: f64,
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitResult) -> Option<(Vec3, Ray)> {
        let direction = hit_record.normal.clone() + Vec3::random_unit();
        Some((
            self.albedo.clone(),
            Ray {
                origin: hit_record.p.clone(),
                direction,
            },
        ))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitResult) -> Option<(Vec3, Ray)> {
        let direction = ray_in.direction.unit().reflect(hit_record.normal.clone())
            + Vec3::random_in_unit_sphere() * self.fuzzy;
        let scattered = Ray {
            origin: hit_record.p.clone(),
            direction,
        };
        if scattered.direction.clone() * hit_record.normal.clone() <= 0.0 {
            return None;
        }
        Some((self.albedo.clone(), scattered))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitResult) -> Option<(Vec3, Ray)> {
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_vector = ray_in.direction.unit();
        let cos_theta = (-unit_vector.clone() * hit_record.normal.clone()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect = if etai_over_etat * sin_theta > 1.0 {
            true
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            random_double() < reflect_prob
        };
        if reflect {
            let direction = unit_vector.reflect(hit_record.normal.clone());
            let scattered = Ray {
                origin: hit_record.p.clone(),
                direction,
            };
            Some((Vec3::ones(), scattered))
        } else {
            let direction = unit_vector.refract(hit_record.normal.clone(), etai_over_etat);
            let scattered = Ray {
                origin: hit_record.p.clone(),
                direction,
            };
            Some((Vec3::ones(), scattered))
        }
    }
}