pub use crate::material::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

use core::f64::INFINITY;
use std::f64::consts::PI;

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
    pub fu: f64,
    pub fv: f64,
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
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
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
                let mut fu = 0.0;
                let mut fv = 0.0;

                HitResult::get_sphere_uv(normal.clone(), &mut fu, &mut fv);
                let mut front_face = false;
                HitResult::set_face_normal(ra, &mut normal, &mut front_face);
                let mat_ptr = self.mat_ptr.clone();
                return Option::Some(HitResult {
                    t,
                    p,
                    fu,
                    fv,
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

                let mut fu = 0.0;
                let mut fv = 0.0;
                HitResult::get_sphere_uv(normal.clone(), &mut fu, &mut fv);

                let mut front_face = false;
                HitResult::set_face_normal(ra, &mut normal, &mut front_face);
                let mat_ptr = self.mat_ptr.clone();
                return Option::Some(HitResult {
                    t,
                    p,
                    fu,
                    fv,
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

/*pub struct Triangle {
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
}*/

pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Hitable for XyRect {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let t = (self.k - ra.origin.z) / ra.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ra.origin.x + t * ra.direction.x;
        let y = ra.origin.y + t * ra.direction.y;
        let fu = (x - self.x0) / (self.x1 - self.x0);
        let fv = (y - self.y0) / (self.y1 - self.y0);
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut normal = Vec3::new(0.0, 0.0, 1.0);
        let mut front_face = false;
        HitResult::set_face_normal(ra, &mut normal, &mut front_face);
        Some(HitResult {
            t,
            p: ra.at(t),
            fu,
            fv,
            normal,
            front_face,
            mat_ptr: self.mat_ptr.clone(),
        })
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

pub struct XzRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Hitable for XzRect {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let t = (self.k - ra.origin.y) / ra.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ra.origin.x + t * ra.direction.x;
        let z = ra.origin.z + t * ra.direction.z;
        let fu = (x - self.x0) / (self.x1 - self.x0);
        let fv = (z - self.z0) / (self.z1 - self.z0);
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut normal = Vec3::new(0.0, 1.0, 0.0);
        let mut front_face = false;
        HitResult::set_face_normal(ra, &mut normal, &mut front_face);
        Some(HitResult {
            t,
            p: ra.at(t),
            fu,
            fv,
            normal,
            front_face,
            mat_ptr: self.mat_ptr.clone(),
        })
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}
pub struct YzRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Hitable for YzRect {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let t = (self.k - ra.origin.x) / ra.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let z = ra.origin.z + t * ra.direction.z;
        let y = ra.origin.y + t * ra.direction.y;
        let fu = (y - self.y0) / (self.y1 - self.y0);
        let fv = (z - self.z0) / (self.z1 - self.z0);
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut normal = Vec3::new(1.0, 0.0, 0.0);
        let mut front_face = false;
        HitResult::set_face_normal(ra, &mut normal, &mut front_face);
        Some(HitResult {
            t,
            p: ra.at(t),
            fu,
            fv,
            normal,
            front_face,
            mat_ptr: self.mat_ptr.clone(),
        })
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}

pub struct Cube {
    pub p0: Vec3,
    pub p1: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub sides: Vec<Arc<dyn Hitable>>,
}
impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        let sides: Vec<Arc<dyn Hitable>> = vec![
            Arc::new(XyRect {
                x0: p0.x,
                x1: p1.x,
                y0: p0.y,
                y1: p1.y,
                k: p1.z,
                mat_ptr: mat_ptr.clone(),
            }),
            Arc::new(XyRect {
                x0: p0.x,
                x1: p1.x,
                y0: p0.y,
                y1: p1.y,
                k: p0.z,
                mat_ptr: mat_ptr.clone(),
            }),
            Arc::new(XzRect {
                x0: p0.x,
                x1: p1.x,
                z0: p0.z,
                z1: p1.z,
                k: p1.y,
                mat_ptr: mat_ptr.clone(),
            }),
            Arc::new(XzRect {
                x0: p0.x,
                x1: p1.x,
                z0: p0.z,
                z1: p1.z,
                k: p0.y,
                mat_ptr: mat_ptr.clone(),
            }),
            Arc::new(YzRect {
                y0: p0.y,
                y1: p1.y,
                z0: p0.z,
                z1: p1.z,
                k: p1.x,
                mat_ptr: mat_ptr.clone(),
            }),
            Arc::new(YzRect {
                y0: p0.y,
                y1: p1.y,
                z0: p0.z,
                z1: p1.z,
                k: p0.x,
                mat_ptr: mat_ptr.clone(),
            }),
        ];
        Self {
            p0,
            p1,
            mat_ptr,
            sides,
        }
    }
}
impl Hitable for Cube {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        hit(&self.sides, &ra, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.p0.clone(),
            max: self.p1.clone(),
        })
    }
}

pub struct Translate {
    pub offset: Vec3,
    pub ptr: Arc<dyn Hitable>,
}
impl Hitable for Translate {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let moved_r = Ray {
            origin: ra.origin.clone() - self.offset.clone(),
            direction: ra.direction.clone(),
        };
        if let Some(mut hit_result) = self.ptr.hit(&moved_r, t_min, t_max) {
            hit_result.p += self.offset.clone();
            HitResult::set_face_normal(
                &moved_r,
                &mut hit_result.normal,
                &mut hit_result.front_face,
            );
            return Some(hit_result);
        };
        None
    }
    fn bounding_box(&self) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box() {
            return Some(AABB {
                min: output_box.min + self.offset.clone(),
                max: output_box.max + self.offset.clone(),
            });
        };
        None
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hitable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<AABB>,
}
impl RotateY {
    pub fn new(ptr: Arc<dyn Hitable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = if let Some(bbox) = ptr.bounding_box() {
            let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
            let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let xx = (i as f64) * bbox.max.x + (1.0 - (i as f64)) * bbox.min.x;
                        let yy = (j as f64) * bbox.max.y + (1.0 - (j as f64)) * bbox.min.y;
                        let zz = (k as f64) * bbox.max.z + (1.0 - (k as f64)) * bbox.min.z;

                        let newx = cos_theta * xx + sin_theta * zz;
                        let newz = -sin_theta * xx + cos_theta * zz;

                        let tester = Vec3::new(newx, yy, newz);
                        min = min.min(tester.clone());
                        max = max.max(tester);
                    }
                }
            }
            Some(AABB { min, max })
        } else {
            None
        };
        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
    fn rotate1(&self, p: &mut Vec3) {
        let q = p.clone();
        p.x = self.cos_theta * q.x - self.sin_theta * q.z;
        p.z = self.sin_theta * q.x + self.cos_theta * q.z;
    }
    fn rotate2(&self, p: &mut Vec3) {
        let q = p.clone();
        p.x = self.cos_theta * q.x + self.sin_theta * q.z;
        p.z = -self.sin_theta * q.x + self.cos_theta * q.z;
    }
}
impl Hitable for RotateY {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut origin = ra.origin.clone();
        let mut direction = ra.direction.clone();

        self.rotate1(&mut origin);
        self.rotate1(&mut direction);

        let rotated_r = Ray { origin, direction };
        if let Some(mut hit_result) = self.ptr.hit(&rotated_r, t_min, t_max) {
            self.rotate2(&mut hit_result.p);
            self.rotate2(&mut hit_result.normal);
            HitResult::set_face_normal(
                &rotated_r,
                &mut hit_result.normal,
                &mut hit_result.front_face,
            );
            return Some(hit_result);
        }
        None
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.bbox.clone()
    }
}

pub struct ConstantMedium {
    pub density: f64,
    pub boundary: Arc<dyn Hitable>,
    pub phase_function: Arc<dyn Material>,
}
impl Hitable for ConstantMedium {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        if let Some(mut rec1) = self.boundary.hit(&ra, -INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(&ra, rec1.t + 0.0001, INFINITY) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);
                if rec1.t >= rec2.t {
                    return None;
                }
                rec1.t = rec1.t.max(0.0);

                let ray_length = ra.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = (-1.0 / self.density) * random_double().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                return Some(HitResult {
                    t,
                    p: ra.at(t),
                    fu: 0.0,
                    fv: 0.0,
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    front_face: true,
                    mat_ptr: self.phase_function.clone(),
                });
            }
        }
        None
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
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
        _ => Arc::new(Cube::new(
            center.clone() - Vec3::new(size, size, size),
            center + Vec3::new(size, size, size),
            random_material(),
        )),
        //_ => Arc::new(Cube::new(center, size, random_material())),
    }
}
