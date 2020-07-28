pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

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
    pub fn hit(&self, ra: &Ray, tmin: &mut f64, tmax: &mut f64) -> bool {
        fn calc(
            min: f64,
            max: f64,
            origin: f64,
            direction: f64,
            tmax: &mut f64,
            tmin: &mut f64,
        ) -> bool {
            let t0 = ffmin((min - origin) / direction, (max - origin) / direction);
            let t1 = ffmax((min - origin) / direction, (max - origin) / direction);
            tmin.max(t0);
            tmax.min(t1);
            tmax > tmin
        }
        calc(
            self.min.x,
            self.max.x,
            ra.origin.x,
            ra.direction.x,
            tmin,
            tmax,
        ) && calc(
            self.min.y,
            self.max.y,
            ra.origin.y,
            ra.direction.y,
            tmin,
            tmax,
        ) && calc(
            self.min.z,
            self.max.z,
            ra.origin.z,
            ra.direction.z,
            tmin,
            tmax,
        )
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

pub struct HitResult<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat_ptr: &'a Box<dyn Material>,
}

impl HitResult<'_> {
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
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        None
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Box<dyn Material>,
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
                let mat_ptr = &(self.mat_ptr);
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
                let mat_ptr = &(self.mat_ptr);
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
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center.clone() - Vec3::ones() * self.radius,
            max: self.center.clone() + Vec3::ones() * self.radius,
        })
    }
}
