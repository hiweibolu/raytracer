use std::vec::Vec;

pub use crate::hit::*;

#[derive(Default)]
pub struct World {
    pub hitlist: Vec<Box<dyn Hitable>>,
}

impl Hitable for World {
    fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut ans: Option<HitResult> = Option::None;
        let mut closest_t = t_max;
        for i in &self.hitlist {
            let opt = i.hit(&ra, t_min, closest_t);
            if let Option::Some(hit_result) = opt {
                closest_t = hit_result.t;
                ans = Option::Some(hit_result);
            }
        }
        ans
    }
}
