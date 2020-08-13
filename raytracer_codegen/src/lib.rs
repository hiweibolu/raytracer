#![allow(clippy::all)]

extern crate proc_macro;
mod hit;
mod random;
mod vec3;
mod world;

use hit::*;
use quote::quote;
use std::sync::Arc;
use vec3::*;
use world::*;

#[proc_macro]
pub fn make_root(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut hitlist: Vec<Arc<dyn Hitable>> = vec![
        Arc::new(YzRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat_ptr: quote! {green.clone()},
        }),
        Arc::new(YzRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            mat_ptr: quote! {red.clone()},
        }),
        Arc::new(XzRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            mat_ptr: quote! {light.clone()},
        }),
        Arc::new(XzRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            mat_ptr: quote! {white.clone()},
        }),
        Arc::new(XzRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat_ptr: quote! {white.clone()},
        }),
        Arc::new(XyRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            mat_ptr: quote! {white.clone()},
        }),
    ];
    let mut cube1: Arc<dyn Hitable> = Arc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        quote! {metal.clone()},
    ));
    cube1 = Arc::new(RotateY::new(cube1, 15.0));
    cube1 = Arc::new(Translate {
        offset: Vec3::new(265.0, 0.0, 295.0),
        ptr: cube1,
    });
    hitlist.push(cube1);

    let mut cube2: Arc<dyn Hitable> = Arc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(175.0, 175.0, 175.0),
        quote! {green.clone()},
    ));
    cube2 = Arc::new(RotateY::new(cube2, -18.0));
    cube2 = Arc::new(Translate {
        offset: Vec3::new(10.0, 300.0, 265.0),
        ptr: cube2,
    });
    hitlist.push(Arc::new(ConstantMedium {
        density: 0.01,
        boundary: cube2,
        phase_function: quote! {Arc::new(Isotropic {
            albedo: Arc::new(ConstantTexture {
                color: Vec3::new(0.2, 0.4, 0.9),
            }),
        })},
    }));

    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(190.0, 90.0, 190.0),
        radius: 90.0,
        mat_ptr: quote! {glass.clone()},
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(278.0, 420.0, 520.0),
        radius: 90.0,
        mat_ptr: quote! {noise.clone()},
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(450.0, 420.0, 520.0),
        radius: 90.0,
        mat_ptr: quote! {image.clone()},
    }));

    let length = hitlist.len();
    let root = Arc::new(BVHNode::new(&mut hitlist, 0, length));

    let token = root.code();
    let result = proc_macro::TokenStream::from(quote! {
        #token
    });

    // uncomment this statement if you want to inspect result
    // println!("{}", result);

    result
}
