pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

use std::sync::Arc;

pub fn ffmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn ffmax(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}
impl AABB {
    pub fn hit(&self, ra: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let t0 = ffmin(
                (self.min[a] - ra.origin[a]) / ra.direction[a],
                (self.max[a] - ra.origin[a]) / ra.direction[a],
            );
            let t1 = ffmax(
                (self.min[a] - ra.origin[a]) / ra.direction[a],
                (self.max[a] - ra.origin[a]) / ra.direction[a],
            );
            if tmin > t1 || tmax < t0 {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let min = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let max = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB { min, max }
    }
}

pub struct HitResult {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
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

pub trait Hitable {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
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
                let mat_ptr = self.mat_ptr.clone();
                return Option::Some(HitResult {
                    t,
                    p,
                    normal,
                    front_face,
                    mat_ptr,
                });
            }
            let root = (-b + temp) / (2.0 * a);
            if root > t_min && root < t_max {
                let t = root;
                let p = ra.at(t);
                let mut normal = (p.clone() - self.center.clone()) / self.radius;
                let mut front_face = false;
                HitResult::set_face_normal(ra, &mut normal, &mut front_face);
                let mat_ptr = self.mat_ptr.clone();
                return Option::Some(HitResult {
                    t,
                    p,
                    normal,
                    front_face,
                    mat_ptr,
                });
            }
        }
        Option::None
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center.clone() - Vec3::ones() * self.radius,
            max: self.center.clone() + Vec3::ones() * self.radius,
        })
    }
}
