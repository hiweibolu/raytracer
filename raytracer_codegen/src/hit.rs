#![allow(clippy::all)]

pub use crate::vec3::Vec3;
use proc_macro2::TokenStream;
use quote::quote;

use core::f64::INFINITY;

use std::sync::Arc;

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}
impl AABB {
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

pub trait Hitable {
    fn bounding_box(&self) -> Option<AABB>;
    fn code(&self) -> TokenStream;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: TokenStream,
}

impl Hitable for Sphere {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center.clone() - Vec3::ones() * self.radius,
            max: self.center.clone() + Vec3::ones() * self.radius,
        })
    }
    fn code(&self) -> TokenStream {
        let center = self.center.code();
        let radius = self.radius;
        let mat_ptr = self.mat_ptr.clone();
        quote! {
            Arc::new(Sphere{
                center: #center,
                radius: #radius,
                mat_ptr: #mat_ptr,
            })
        }
    }
}

pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat_ptr: TokenStream,
}

impl Hitable for XyRect {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
    fn code(&self) -> TokenStream {
        let mat_ptr = self.mat_ptr.clone();
        let x0 = self.x0;
        let x1 = self.x1;
        let y0 = self.y0;
        let y1 = self.y1;
        let k = self.k;
        quote! {
            Arc::new(XyRect{
                x0: #x0,
                x1: #x1,
                y0: #y0,
                y1: #y1,
                k: #k,
                mat_ptr: #mat_ptr,
            })
        }
    }
}

pub struct XzRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat_ptr: TokenStream,
}

impl Hitable for XzRect {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
    fn code(&self) -> TokenStream {
        let mat_ptr = self.mat_ptr.clone();
        let x0 = self.x0;
        let x1 = self.x1;
        let z0 = self.z0;
        let z1 = self.z1;
        let k = self.k;
        quote! {
            Arc::new(XzRect{
                x0: #x0,
                x1: #x1,
                z0: #z0,
                z1: #z1,
                k: #k,
                mat_ptr: #mat_ptr,
            })
        }
    }
}
pub struct YzRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat_ptr: TokenStream,
}

impl Hitable for YzRect {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
    fn code(&self) -> TokenStream {
        let mat_ptr = self.mat_ptr.clone();
        let y0 = self.y0;
        let y1 = self.y1;
        let z0 = self.z0;
        let z1 = self.z1;
        let k = self.k;
        quote! {
            Arc::new(YzRect{
                y0: #y0,
                y1: #y1,
                z0: #z0,
                z1: #z1,
                k: #k,
                mat_ptr: #mat_ptr,
            })
        }
    }
}

pub struct Cube {
    pub p0: Vec3,
    pub p1: Vec3,
    pub mat_ptr: TokenStream,
    pub sides: Vec<Arc<dyn Hitable>>,
}
impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, mat_ptr: TokenStream) -> Self {
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
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.p0.clone(),
            max: self.p1.clone(),
        })
    }
    fn code(&self) -> TokenStream {
        let mat_ptr = self.mat_ptr.clone();
        let p0 = self.p0.code();
        let p1 = self.p1.code();
        quote! {
            Arc::new(Cube::new(
                #p0,
                #p1,
                #mat_ptr
            ))
        }
    }
}

pub struct Translate {
    pub offset: Vec3,
    pub ptr: Arc<dyn Hitable>,
}
impl Hitable for Translate {
    fn bounding_box(&self) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box() {
            return Some(AABB {
                min: output_box.min + self.offset.clone(),
                max: output_box.max + self.offset.clone(),
            });
        };
        None
    }
    fn code(&self) -> TokenStream {
        let offset = self.offset.code();
        let ptr = self.ptr.code();
        quote! {
            Arc::new(Translate{
                offset: #offset,
                ptr: #ptr,
            })
        }
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hitable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub angle: f64,
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
            angle,
        }
    }
}
impl Hitable for RotateY {
    fn bounding_box(&self) -> Option<AABB> {
        self.bbox.clone()
    }
    fn code(&self) -> TokenStream {
        let angle = self.angle;
        let ptr = self.ptr.code();
        quote! {
            Arc::new(
                RotateY::new(
                    #ptr,
                    #angle,
                )
            )
        }
    }
}

pub struct ConstantMedium {
    pub density: f64,
    pub boundary: Arc<dyn Hitable>,
    pub phase_function: TokenStream,
}
impl Hitable for ConstantMedium {
    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
    fn code(&self) -> TokenStream {
        let density = self.density;
        let boundary = self.boundary.code();
        let phase_function = self.phase_function.clone();
        quote! {
            Arc::new(
                ConstantMedium{
                    density: #density,
                    boundary: #boundary,
                    phase_function: #phase_function
                }
            )
        }
    }
}
