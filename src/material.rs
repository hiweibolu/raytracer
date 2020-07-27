pub use crate::hit::HitResult;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

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
        let direction = ray_in.direction.unit().reflect(hit_record.normal.clone());
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
