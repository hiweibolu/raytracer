mod camera;
#[allow(clippy::borrowed_box)]
mod hit;
mod material;
mod random;
mod ray;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
mod world;
use core::f64::INFINITY;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use camera::Camera;
pub use hit::*;
pub use material::*;
pub use random::*;
pub use ray::Ray;
pub use texture::*;
pub use vec3::Vec3;
pub use world::World;

const WIDTH: u32 = 1280;
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
    let unit = ra.direction.unit();
    let t = (unit.y + 1.0) * 0.5;
    Vec3::lerp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.25, 0.35, 0.5), t)
}

fn oneweekend_world() -> World {
    let mut hitlist: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat_ptr: Box::new(Lambertian {
                albedo: Box::new(CheckerTexture {
                    odd: Box::new(ConstantTexture {
                        color: Vec3::new(0.2, 0.3, 0.1),
                    }),
                    even: Box::new(ConstantTexture {
                        color: Vec3::new(0.9, 0.9, 0.9),
                    }),
                }),
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            mat_ptr: Box::new(Dielectric { ref_idx: 1.5 }),
        }),
        Box::new(Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            mat_ptr: Box::new(Lambertian {
                albedo: Box::new(ConstantTexture {
                    color: Vec3::new(0.4, 0.2, 0.1),
                }),
            }),
        }),
        Box::new(Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            mat_ptr: Box::new(Metal {
                albedo: Box::new(ConstantTexture {
                    color: Vec3::new(0.7, 0.6, 0.5),
                }),
                fuzzy: 0.0,
            }),
        }),
    ];
    for a in -11..11 {
        for b in -11..11 {
            hitlist.push(Box::new({
                let radius = 0.2;
                let center = Vec3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );
                let choose_mat = random_double();
                let mat_ptr: Box<dyn Material> = if choose_mat < 0.2 {
                    let albedo: Box<dyn Texture> = Box::new(ConstantTexture {
                        color: Vec3::random(),
                    });
                    Box::new(Lambertian { albedo })
                } else if choose_mat < 0.4 {
                    let albedo: Box<dyn Texture> = Box::new(ConstantTexture {
                        color: Vec3::random_range(0.5, 1.0),
                    });
                    let fuzzy = random_double_range(0.0, 0.5);
                    Box::new(Metal { albedo, fuzzy })
                } else if choose_mat < 0.7 {
                    Box::new(Dielectric { ref_idx: 1.5 })
                } else {
                    let emit: Box<dyn Texture> = Box::new(ConstantTexture {
                        color: Vec3::random_range(0.5, 1.0),
                    });
                    Box::new(DiffuseLight { emit })
                };
                Sphere {
                    center,
                    radius,
                    mat_ptr,
                }
            }))
        }
    }
    World { hitlist }
}

fn oneweekend(cam: &Camera) {
    let mut img: RgbImage = ImageBuffer::new(cam.width, cam.height);
    let bar = ProgressBar::new(cam.width as u64);

    let wor = oneweekend_world();
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
                    color += ray_color(ra, &wor, MAX_DEPTH) / (sample_number as f64);
                }
            }

            let pixel = img.get_pixel_mut(x, cam.height - 1 - y);
            *pixel = image::Rgb(color.sqrt().color());
        }
        bar.inc(1);
    }

    img.save("output/oneweekend.png").unwrap();
    bar.finish();
}

fn main() {
    let cam = Camera::new(
        30f64.to_radians(),
        16.0 / 9.0,
        WIDTH,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.001,
    );
    oneweekend(&cam);
}
