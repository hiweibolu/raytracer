pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

pub struct HitResult {
    //pub color: Vec3,
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitResult {
    pub fn set_face_normal(ra: &Ray, normal: &mut Vec3, front_face: &mut bool) {
        *front_face = ra.direction.clone() * normal.clone() < 0.0;
        *normal = if *front_face {
            normal.clone()
        } else {
            -normal.clone()
        };
    }
}

pub enum Option<HitResult> {
    Some(HitResult),
    None,
}

pub trait Hitable {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let (delta, a, b) = {
            let oc = ra.origin.clone() - self.center.clone();
            let a = ra.direction.squared_length();
            let b = oc.clone() * ra.direction.clone() * 2.0;
            let c = oc.squared_length() - self.radius * self.radius;
            (b * b - 4.0 * a * c, a, b)
        };
        if delta > 0.0 {
            let temp = delta.sqrt();
            let root = (-b - temp) / (2.0 * a);
            if root > t_min && root < t_max {
                let t = root;
                let p = ra.at(t);
                let mut normal = (p.clone() - self.center.clone()) / self.radius;
                let mut front_face = false;
                HitResult::set_face_normal(ra, &mut normal, &mut front_face);
                return Option::Some(HitResult {
                    t,
                    p,
                    normal,
                    front_face,
                });
            }
            let root = (-b + temp) / (2.0 * a);
            if root > t_min && root < t_max {
                let t = root;
                let p = ra.at(t);
                let mut normal = (p.clone() - self.center.clone()) / self.radius;
                let mut front_face = false;
                HitResult::set_face_normal(ra, &mut normal, &mut front_face);
                return Option::Some(HitResult {
                    t,
                    p,
                    normal,
                    front_face,
                });
            }
        }
        Option::None
        /*if root < t_min || root > t_max {
            return Option::None;
        }
        let normal = (ra.at(root) - self.center.clone()).unit();
        Option::Some(HitResult {
            color: Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5,
        })*/
    }
}
