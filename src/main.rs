mod camera;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use camera::Camera;
pub use ray::Ray;
pub use vec3::Vec3;

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
*/

fn ray_color(ra: Ray) -> Vec3 {
    let unit = ra.direction.unit();
    let t = (unit.y + 1.0) * 0.5;
    Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
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

fn oneweekend(cam: &Camera) {
    let mut img: RgbImage = ImageBuffer::new(cam.width, cam.height);
    let bar = ProgressBar::new(cam.width as u64);

    for x in 0..cam.width {
        for y in 0..cam.height {
            let u = (x as f64) / ((cam.width - 1) as f64);
            let v = (y as f64) / ((cam.height - 1) as f64);
            let ra = Ray {
                origin: cam.position.clone(),
                direction: cam.lower_left_corner() + cam.horizontal() * u + cam.vertical() * v
                    - cam.position.clone(),
            };

            let pixel = img.get_pixel_mut(x, y);
            let color = ray_color(ra);
            *pixel = image::Rgb(color.color());
        }
        bar.inc(1);
    }

    img.save("output/oneweekend.png").unwrap();
    bar.finish();
}

fn main() {
    let cam = Camera::new(16.0 / 9.0, 1280, Vec3::zero(), 4.0, 1.0);
    firework();
    oneweekend(&cam);
}
