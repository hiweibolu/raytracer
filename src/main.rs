mod camera;
mod hit;
mod random;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
mod world;
use core::f64::INFINITY;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use camera::Camera;
pub use hit::*;
pub use random::*;
pub use ray::Ray;
pub use vec3::Vec3;
pub use world::World;

const ANTIALIASING: i32 = 2;
const MAX_DEPTH: i32 = 50;

fn ray_color(ra: Ray, wor: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let opt: Option<HitResult> = wor.hit(&ra, 0.001, INFINITY);
    if let Option::Some(hit_result) = opt {
        let target = hit_result.p.clone() + hit_result.normal.clone() + Vec3::random_unit();
        //return (hit_result.normal + Vec3::ones()) * 0.5;
        return ray_color(
            Ray {
                origin: hit_result.p.clone(),
                direction: target - hit_result.p,
            },
            &wor,
            depth - 1,
        ) * 0.5;
    }
    let unit = ra.direction.unit();
    let t = (unit.y + 1.0) * 0.5;
    Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

fn oneweekend(cam: &Camera) {
    let mut img: RgbImage = ImageBuffer::new(cam.width, cam.height);
    let bar = ProgressBar::new(cam.width as u64);

    let wor = World {
        hitlist: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -2.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -2.0),
                radius: 100.0,
            }),
        ],
    };

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
    let cam = Camera::new(16.0 / 9.0, 1280, Vec3::zero(), 4.0, 1.0);
    oneweekend(&cam);
}
