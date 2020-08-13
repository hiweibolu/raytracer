#![allow(clippy::all)]

use proc_macro2::TokenStream;
use quote::quote;
use std::sync::Arc;
use std::vec::Vec;

pub use crate::hit::*;
pub use crate::random::*;
use std::cmp::Ordering;

pub struct BVHNode {
    pub bbox: AABB,
    pub left: Arc<dyn Hitable>,
    pub right: Arc<dyn Hitable>,
}

pub fn cmp(one: &Arc<dyn Hitable>, other: &Arc<dyn Hitable>, a: i32) -> Ordering {
    let opt_left = one.bounding_box();
    let opt_right = other.bounding_box();
    if let Some(box_left) = opt_left {
        if let Some(box_right) = opt_right {
            if let Some(cmp) = box_left.min[a].partial_cmp(&box_right.min[a]) {
                return cmp;
            }
        }
    }
    panic!("no box to compare!");
}

impl BVHNode {
    pub fn new(hitlist: &mut Vec<Arc<dyn Hitable>>, l: usize, r: usize) -> Self {
        let axis = (3.0 * random_double()).floor() as i32;
        hitlist[l..r].sort_by(|a, b| cmp(a, b, axis));
        let (left, right): (Arc<dyn Hitable>, Arc<dyn Hitable>) = if r - l == 1 {
            (hitlist[l].clone(), hitlist[l].clone())
        } else if r - l == 2 {
            (hitlist[l].clone(), hitlist[l + 1].clone())
        } else {
            let mid = (l + r) / 2;
            (
                Arc::new(Self::new(hitlist, l, mid)),
                Arc::new(Self::new(hitlist, mid, r)),
            )
        };
        let opt_left = left.bounding_box();
        let opt_right = right.bounding_box();
        if let Some(box_left) = opt_left {
            if let Some(box_right) = opt_right {
                return Self {
                    left,
                    right,
                    bbox: AABB::surrounding_box(box_left, box_right),
                };
            }
        }
        panic!("no box to bounding!");
    }
}
impl Hitable for BVHNode {
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox.clone())
    }
    fn code(&self) -> TokenStream {
        let left = self.left.code();
        let right = self.right.code();
        let min = self.bbox.min.code();
        let max = self.bbox.max.code();
        quote! {
            Arc::new(
                BVHNode{
                    bbox: AABB{
                        min: #min,
                        max: #max,
                    },
                    left: #left,
                    right: #right,
                }
            )
        }
    }
}

/*pub struct World {
    pub hitlist: Vec<Arc<dyn Hitable>>,
    pub root: Arc<dyn Hitable>,
}

impl World {
    pub fn hit(&self, ra: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        self.root.hit(&ra, t_min, t_max)
    }
    pub fn new(mut hitlist: Vec<Arc<dyn Hitable>>) -> Self {
        let length = hitlist.len();
        let root = Arc::new(BVHNode::new(&mut hitlist, 0, length));
        Self { hitlist, root }
    }
}*/
