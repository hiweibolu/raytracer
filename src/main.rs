mod camera;
mod hit;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
mod world;
use core::f64::INFINITY;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use camera::Camera;
pub use hit::*;
pub use ray::Ray;
pub use vec3::Vec3;
pub use world::World;

const ANTIALIASING: i32 = 10;

fn ray_color(ra: Ray, wor: &World) -> Vec3 {
    let opt: Option<HitResult> = wor.hit(&ra, 0.0, INFINITY);
    if let Option::Some(hit_result) = opt {
        return (hit_result.normal + Vec3::ones()) * 0.5;
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
                    color += ray_color(ra, &wor) / (sample_number as f64);
                }
            }

            let pixel = img.get_pixel_mut(x, cam.height - 1 - y);
            *pixel = image::Rgb(color.color());
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

/*
fn test() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color = (x / 4) as u8;
            *pixel = image::Rgb([color, color, color]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn setu() {
    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    let blue: u8 = 100;

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([(x / 4) as u8, (y / 2) as u8, blue]);
        }
        bar.inc(1);
    }

    img.save("output/setu.png").unwrap();
    bar.finish();
}

fn firework() {
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..1024 {
            let mut rgb = image::Rgb([128 + (x / 9) as u8, 150 + (y / 10) as u8, 255]);

            let max_length = 10000.0;
            let mut comp = (
                ((x as f64) / 1023.0 - 0.5) * 2.0,
                ((y as f64) / 1023.0 - 0.5) * 2.0,
            );

            fn julia(com: (f64, f64)) -> (f64, f64) {
                (
                    com.0 * com.0 - com.1 * com.1 - 0.8,
                    com.0 * com.1 + com.1 * com.0 + 0.156,
                )
            }

            let mut flag = false;
            for _i in 0..200 {
                comp = julia(comp);
                if comp.0 * comp.0 + comp.1 * comp.1 > max_length {
                    flag = true;
                    break;
                }
            }

            if !flag {
                rgb = image::Rgb([255, 188 + (comp.0 as u8), 160 + (comp.1 as u8)]);
            }

            let pixel = img.get_pixel_mut(x, y);
            *pixel = rgb;
        }
        bar.inc(1);
    }

    img.save("output/firework.png").unwrap();
    bar.finish();
}
*/
