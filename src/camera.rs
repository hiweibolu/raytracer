pub use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub ratio: f64,
    pub width: u32,
    pub height: u32,
    pub position: Vec3,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
}

impl Camera {
    pub fn new(rat: f64, wid: u32, pos: Vec3, vwid: f64, flen: f64) -> Self {
        Self {
            ratio: rat,
            width: wid,
            height: ((wid as f64) / rat) as u32,
            position: pos,
            viewport_width: vwid,
            viewport_height: vwid / rat,
            focal_length: flen,
        }
    }
    pub fn horizontal(&self) -> Vec3 {
        Vec3 {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn vertical(&self) -> Vec3 {
        Vec3 {
            x: 0.0,
            y: self.viewport_height,
            z: 0.0,
        }
    }
    pub fn lower_left_corner(&self) -> Vec3 {
        self.position.clone()
            - self.horizontal() * 0.5
            - self.vertical() * 0.5
            - Vec3::new(0.0, 0.0, self.focal_length)
    }
}
