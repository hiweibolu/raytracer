pub use crate::material::*;
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

pub fn hit(hitlist: &[Arc<dyn Hitable>], ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
    let mut ans: Option<HitResult> = None;
    let mut closest_t = t_max;
    for i in hitlist {
        let opt = i.hit(&ra, t_min, closest_t);
        if let Option::Some(hit_result) = opt {
            closest_t = hit_result.t;
            ans = Option::Some(hit_result);
        }
    }
    ans
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

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub mat_ptr: Arc<dyn Material>,
}
impl Hitable for Triangle {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let e1 = self.v1.clone() - self.v0.clone();
        let e2 = self.v2.clone() - self.v0.clone();
        let phi = Vec3::cross(ra.direction.clone(), e2.clone());
        let mut det = e1.clone() * phi.clone();
        let temp = if det > 0.0 {
            ra.origin.clone() - self.v0.clone()
        } else {
            det = -det;
            self.v0.clone() - ra.origin.clone()
        };
        if det < 0.0 {
            return None;
        }
        let u = temp.clone() * phi;
        if u < 0.0 || u > det {
            return None;
        }
        let qq = Vec3::cross(temp, e1.clone());
        let v = ra.direction.clone() * qq.clone();
        if v < 0.0 || u + v > det {
            return None;
        }
        let mut t = e2.clone() * qq;
        let invdet = 1.0 / det;
        t *= invdet;
        if t < t_min || t > t_max {
            return None;
        }
        /*u *= invdet;
        v *= invdet;*/
        let p = ra.at(t);
        let mut normal = Vec3::cross(e1, e2).unit();
        let mut front_face = false;
        HitResult::set_face_normal(ra, &mut normal, &mut front_face);
        let mat_ptr = self.mat_ptr.clone();
        Some(HitResult {
            t,
            p,
            normal,
            front_face,
            mat_ptr,
        })
    }
}

pub struct Cube {
    pub hitlist: Vec<Arc<dyn Hitable>>,
    pub bbox: AABB,
    /*pub v0: Vec3,
    pub x_diff: Vec3,
    pub y_diff: Vec3,
    pub z_diff: Vec3,
    pub mat_ptr: Arc<dyn Material>,*/
}
impl Cube {
    pub fn build(
        v0: Vec3,
        x_diff: Vec3,
        y_diff: Vec3,
        z_diff: Vec3,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let v1 = v0.clone() + x_diff.clone() + y_diff.clone() + z_diff.clone();
        let mut min = v0.clone();
        min = min.min(v0.clone() + x_diff.clone());
        min = min.min(v0.clone() + y_diff.clone());
        min = min.min(v0.clone() + z_diff.clone());
        min = min.min(v0.clone() + x_diff.clone() + y_diff.clone());
        min = min.min(v0.clone() + y_diff.clone() + z_diff.clone());
        min = min.min(v0.clone() + z_diff.clone() + x_diff.clone());
        min = min.min(v0.clone() + x_diff.clone() + y_diff.clone() + z_diff.clone());
        let mut max = v0.clone();
        max = max.max(v0.clone() + x_diff.clone());
        max = max.max(v0.clone() + y_diff.clone());
        max = max.max(v0.clone() + z_diff.clone());
        max = max.max(v0.clone() + x_diff.clone() + y_diff.clone());
        max = max.max(v0.clone() + y_diff.clone() + z_diff.clone());
        max = max.max(v0.clone() + z_diff.clone() + x_diff.clone());
        max = max.max(v0.clone() + x_diff.clone() + y_diff.clone() + z_diff.clone());
        Self {
            hitlist: vec![
                Arc::new(Triangle {
                    v0: v0.clone(),
                    v1: v0.clone() + x_diff.clone(),
                    v2: v0.clone() + y_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v0.clone() + x_diff.clone() + y_diff.clone(),
                    v1: v0.clone() + y_diff.clone(),
                    v2: v0.clone() + x_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v0.clone(),
                    v1: v0.clone() + y_diff.clone(),
                    v2: v0.clone() + z_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v0.clone() + y_diff.clone() + z_diff.clone(),
                    v1: v0.clone() + z_diff.clone(),
                    v2: v0.clone() + y_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v0.clone(),
                    v1: v0.clone() + z_diff.clone(),
                    v2: v0.clone() + x_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v0.clone() + x_diff.clone() + z_diff.clone(),
                    v1: v0.clone() + x_diff.clone(),
                    v2: v0 + z_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone(),
                    v1: v1.clone() - y_diff.clone(),
                    v2: v1.clone() - x_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone() - x_diff.clone() - y_diff.clone(),
                    v1: v1.clone() - x_diff.clone(),
                    v2: v1.clone() - y_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone(),
                    v1: v1.clone() - z_diff.clone(),
                    v2: v1.clone() - y_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone() - y_diff.clone() - z_diff.clone(),
                    v1: v1.clone() - y_diff,
                    v2: v1.clone() - z_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone(),
                    v1: v1.clone() - x_diff.clone(),
                    v2: v1.clone() - z_diff.clone(),
                    mat_ptr: mat_ptr.clone(),
                }),
                Arc::new(Triangle {
                    v0: v1.clone() - x_diff.clone() - z_diff.clone(),
                    v1: v1.clone() - z_diff,
                    v2: v1 - x_diff,
                    mat_ptr,
                }),
            ],
            bbox: AABB { min, max },
        }
    }
    pub fn new(center: Vec3, size: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self::build(
            center - Vec3::new(size, size, size),
            Vec3::new(size * 2.0, 0.0, 0.0),
            Vec3::new(0.0, size * 2.0, 0.0),
            Vec3::new(0.0, 0.0, size * 2.0),
            mat_ptr,
        )
    }
}
impl Hitable for Cube {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        hit(&self.hitlist, &ra, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

pub fn random_hitable(center: Vec3, size: f64) -> Arc<dyn Hitable> {
    let choose_hitable = (2.0 * random_double()).floor() as i32;
    match choose_hitable {
        0 => Arc::new(Sphere {
            center,
            radius: size,
            mat_ptr: random_material(),
        }),
        _ => Arc::new(Cube::new(center, size, random_material())),
    }
}
