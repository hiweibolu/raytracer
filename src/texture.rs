pub use crate::perlin::Perlin;
pub use crate::random::*;
pub use crate::vec3::Vec3;

use image::{open, RgbImage};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3,
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
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

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        //Vec3::ones() * 0.5 * (1.0 + self.noise.noise(p * self.scale))
        Vec3::ones() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p)).sin())
    }
}

pub struct ImageTexture {
    pub img: RgbImage,
    pub width: usize,
    pub height: usize,
}
impl ImageTexture {
    pub fn new() -> Self {
        match open("src/earthmap.jpg") {
            Err(why) => panic!("{:?}", why),
            Ok(img) => Self {
                img: img.to_rgb(),
                width: 1024,
                height: 512,
            },
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        //Vec3::ones() * 0.5 * (1.0 + self.noise.noise(p * self.scale))

        let fu = u.max(0.0).min(1.0);
        let fv = 1.0 - v.max(0.0).min(1.0);

        let mut ii = (fu * self.width as f64).floor() as usize;
        let mut jj = (fv * self.height as f64).floor() as usize;
        ii = ii.min(self.width - 1);
        jj = jj.min(self.height - 1);
        let color_scale = 1.0 / 255.0;

        let pixel = self.img.get_pixel(ii as u32, jj as u32);
        Vec3::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub fn random_texture() -> Arc<dyn Texture> {
    let choose_texture = (2.0 * random_double()).floor() as i32;
    match choose_texture {
        0 => Arc::new(ConstantTexture {
            color: Vec3::random(),
        }),
        _ => Arc::new(CheckerTexture {
            odd: random_texture(),
            even: random_texture(),
        }),
    }
}
