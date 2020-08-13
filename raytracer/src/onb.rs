pub use crate::vec3::Vec3;
pub use std::ops::Index;

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u.clone() * a + self.v.clone() * b + self.w.clone() * c
    }
    pub fn localvec(&self, p: Vec3) -> Vec3 {
        self.u.clone() * p[0] + self.v.clone() * p[1] + self.w.clone() * p[2]
    }
    pub fn build_from_w(normal: Vec3) -> Self {
        let w = normal.unit();
        let temp = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(w.clone(), temp).unit();
        let u = Vec3::cross(w.clone(), v.clone());
        Self { u, v, w }
    }
}
