pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub vfov: f64,
    pub ratio: f64,
    pub width: u32,
    pub height: u32,
    pub position: Vec3,
    pub lookat: Vec3,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub vup: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, ratio: f64, width: u32, position: Vec3, lookat: Vec3, vup: Vec3) -> Self {
        let height = ((width as f64) / ratio) as u32;
        let viewport_height = (vfov * 0.5).tan() * 2.0;
        let viewport_width = viewport_height * ratio;
        let w = (position.clone() - lookat.clone()).unit();
        let u = (Vec3::cross(vup.clone(), w.clone())).unit();
        let v = Vec3::cross(w.clone(), u.clone());
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner =
            position.clone() - horizontal.clone() * 0.5 - vertical.clone() * 0.5 - w;
        Self {
            vfov,
            ratio,
            width,
            height,
            position,
            lookat,
            viewport_height,
            viewport_width,
            lower_left_corner,
            horizontal,
            vertical,
            vup,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.position.clone(),
            direction: self.lower_left_corner.clone()
                + self.horizontal.clone() * u
                + self.vertical.clone() * v
                - self.position.clone(),
        }
    }
}
