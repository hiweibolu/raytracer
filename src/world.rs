use std::vec::Vec;

pub use crate::hit::*;

#[derive(Default)]
pub struct World {
    pub hitlist: Vec<Box<dyn Hitable>>,
}
