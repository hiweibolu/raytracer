mod camera;
#[allow(clippy::borrowed_box)]
mod hit;
mod material;
mod random;
mod ray;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
#[allow(clippy::borrowed_box)]
mod world;
use core::f64::INFINITY;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub use camera::Camera;
pub use hit::*;
pub use material::*;
pub use random::*;
pub use ray::Ray;
pub use texture::*;
pub use vec3::Vec3;
pub use world::World;

const WIDTH: u32 = 600;
const ANTIALIASING: i32 = 10;
const MAX_DEPTH: i32 = 50;

fn ray_color(ra: Ray, wor: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let opt: Option<HitResult> = wor.hit(&ra, 0.001, INFINITY);
    if let Option::Some(hit_result) = opt {
        let option: Option<(Vec3, Ray)> = hit_result.mat_ptr.scatter(&ra, &hit_result);
        let emitted = hit_result.mat_ptr.emitted(0.0, 0.0, hit_result.p.clone());
        if let Option::Some(scatter_result) = option {
            return emitted
                + Vec3::elemul(
                    scatter_result.0,
                    ray_color(scatter_result.1, &wor, depth - 1),
                );
        }
        return emitted;
    }
	
	Vec3::zero()
    /*let unit = ra.direction.unit();
    let t = (unit.y + 1.0) * 0.5;
    Vec3::lerp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), t)*/
}

fn cornell_box() -> World {
    let red = Arc::new(Lambertian {
        albedo: Arc::new(ConstantTexture {
            color: Vec3::new(0.65, 0.05, 0.05),
        }),
    });
    let white = Arc::new(Lambertian {
        albedo: Arc::new(ConstantTexture {
            color: Vec3::new(0.73, 0.73, 0.73),
        }),
    });
    let green = Arc::new(Lambertian {
        albedo: Arc::new(ConstantTexture {
            color: Vec3::new(0.12, 0.45, 0.15),
        }),
    });
    let light = Arc::new(DiffuseLight {
        emit: Arc::new(ConstantTexture {
            color: Vec3::new(15.0, 15.0, 15.0),
        }),
    });
    let mut hitlist: Vec<Arc<dyn Hitable>> = vec![
        Arc::new(YzRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat_ptr: green,
        }),
        Arc::new(YzRect {
            y0: 0.0,
            y1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            mat_ptr: red,
        }),
        Arc::new(XzRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            mat_ptr: light,
        }),
        Arc::new(XzRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
            mat_ptr: white.clone(),
        }),
        Arc::new(XzRect {
            x0: 0.0,
            x1: 555.0,
            z0: 0.0,
            z1: 555.0,
            k: 555.0,
            mat_ptr: white.clone(),
        }),
        Arc::new(XyRect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            mat_ptr: white.clone(),
        }),
    ];
    let mut cube1: Arc<dyn Hitable> = Arc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    cube1 = Arc::new(RotateY::new(cube1, 15.0));
    cube1 = Arc::new(Translate {
        offset: Vec3::new(265.0, 0.0, 295.0),
        ptr: cube1,
    });
    hitlist.push(cube1);

    let mut cube2: Arc<dyn Hitable> = Arc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    cube2 = Arc::new(RotateY::new(cube2, -18.0));
    cube2 = Arc::new(Translate {
        offset: Vec3::new(130.0, 0.0, 65.0),
        ptr: cube2,
    });
    hitlist.push(cube2);
    World::new(hitlist)
}

fn work(cam: &Camera) {
    let mut img: RgbImage = ImageBuffer::new(cam.width, cam.height);
    let bar = ProgressBar::new(cam.width as u64);

    let wor = cornell_box(); //oneweekend_world();
    let length_per_step = [
        1.0 / ((ANTIALIASING + 1) as f64),
        1.0 / ((ANTIALIASING + 1) as f64),
    ];
    let sample_number = ANTIALIASING * ANTIALIASING;
    for x in 0..cam.width {
        for y in 0..cam.height {
            let mut color = Vec3::zero();
            for x_step in 1..ANTIALIASING + 1 {
                for y_step in 1..ANTIALIASING + 1 {
                    let u =
                        ((x as f64) + (x_step as f64) * length_per_step[0]) / (cam.width as f64);
                    let v =
                        ((y as f64) + (y_step as f64) * length_per_step[1]) / (cam.height as f64);
                    let ra = cam.get_ray(u, v);
                    let co = ray_color(ra, &wor, MAX_DEPTH);
                    /*let co = ray_color(ra, &wor, MAX_DEPTH)
                    .min(Vec3::ones())
                    .max(Vec3::zero());*/
                    color += co / (sample_number as f64);
                }
            }

            let pixel = img.get_pixel_mut(x, cam.height - 1 - y);
            color = color.min(Vec3::ones()).max(Vec3::zero());
            *pixel = image::Rgb(color.sqrt().color());
        }
        bar.inc(1);
    }

    img.save("output/output.png").unwrap();
    bar.finish();
}

fn main() {
    let cam = Camera::new(
        40f64.to_radians(),
        1.0,
        WIDTH,
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    work(&cam);
}
