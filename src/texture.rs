pub use crate::random::*;
pub use crate::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3,
}

pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color.clone()
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
