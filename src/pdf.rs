pub use crate::hit::*;
pub use crate::onb::ONB;
pub use crate::random::*;
pub use crate::vec3::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    pub uvw: ONB,
}
impl CosinePdf {
    pub fn new(n: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(n),
        }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.w.clone();
        (cosine / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.localvec(Vec3::random_cosine_direction())
    }
}

pub struct HitablePdf {
    pub origin: Vec3,
    pub ptr: Arc<dyn Hitable>,
}
impl Pdf for HitablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.origin.clone(), direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.origin.clone())
    }
}

pub struct MixturePdf {
    pub p0: Arc<dyn Pdf>,
    pub p1: Arc<dyn Pdf>,
    pub d0: f64,
    pub d1: f64,
}
impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.d0 * self.p0.value(direction.clone()) + self.d1 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double() < self.d0 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
