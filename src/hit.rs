pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

pub struct HitResult {
    pub color: Vec3,
}

pub enum Option<HitResult> {
    Some(HitResult),
    None,
}

pub trait Hitable {
    fn hit(&self, ra: &Ray) -> Option<HitResult>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ra: &Ray) -> Option<HitResult> {
        let oc = ra.origin.clone() - self.center.clone();
        let a = ra.direction.squared_length();
        let b = oc.clone() * ra.direction.clone() * 2.0;
        let c = oc.squared_length() - self.radius * self.radius;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return Option::None;
        }
        let root = (-b - delta.sqrt()) / (2.0 * a);
        let normal = (ra.at(root) - self.center.clone()).unit();
        Option::Some(HitResult {
            color: Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5,
        })
    }
}
