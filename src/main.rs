mod camera;
mod hit;
mod material;
mod onb;
mod pdf;
mod perlin;
mod random;
mod ray;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
mod world;
use core::f64::INFINITY;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub use camera::Camera;
pub use hit::*;
pub use material::*;
pub use onb::ONB;
pub use pdf::*;
pub use random::*;
pub use ray::Ray;
pub use texture::*;
pub use vec3::Vec3;
pub use world::World;

const WIDTH: u32 = 600;
const ANTIALIASING: i32 = 20;
const MAX_DEPTH: i32 = 50;

fn ray_color(ra: Ray, wor: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit_result) = wor.hit(&ra, 0.001, INFINITY) {
        let emitted = hit_result
            .mat_ptr
            .emitted(&hit_result, 0.0, 0.0, hit_result.p.clone());
        if let Some(scatter_result) = hit_result.mat_ptr.scatter(&ra, &hit_result) {
            if scatter_result.3 {
                return Vec3::elemul(
                    scatter_result.0,
                    ray_color(scatter_result.1, &wor, depth - 1),
                );
            }

            let light = Arc::new(DiffuseLight {
                emit: Arc::new(ConstantTexture {
                    color: Vec3::new(15.0, 15.0, 15.0),
                }),
            });
            let light_shape: Arc<dyn Hitable> = Arc::new(XzRect {
                x0: 213.0,
                x1: 343.0,
                z0: 227.0,
                z1: 332.0,
                k: 554.0,
                mat_ptr: light,
            });
            let p0 = HitablePdf {
                origin: hit_result.p.clone(),
                ptr: light_shape,
            };
            let p1 = CosinePdf::new(hit_result.normal.clone());
            let p = MixturePdf {
                p0: Arc::new(p0),
                p1: Arc::new(p1),
                d0: 0.5,
                d1: 0.5,
            };

            let scattered = Ray {
                origin: hit_result.p.clone(),
                direction: p.generate(),
            };
            let pdf_value = p.value(scattered.direction.clone());

            return emitted
                + Vec3::elemul(
                    scatter_result.0
                        * hit_result
                            .mat_ptr
                            .scattering_pdf(&ra, &hit_result, &scattered),
                    ray_color(scattered, &wor, depth - 1),
                ) / pdf_value;
        }
        return emitted;
    }

    Vec3::zero()
    /*let unit = ra.direction.unit();
    let t = (unit.y + 1.0) * 0.5;
    Vec3::lerp(Vec3::new(0.4, 0.6, 0.8), Vec3::new(0.3, 0.5, 0.5), t)*/
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
    let metal = Arc::new(Metal {
        albedo: Arc::new(ConstantTexture {
            color: Vec3::new(0.8, 0.85, 0.88),
        }),
        fuzzy: 0.0,
    });
    let glass = Arc::new(Dielectric { ref_idx: 1.5 });
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
        metal,
    ));
    cube1 = Arc::new(RotateY::new(cube1, 15.0));
    cube1 = Arc::new(Translate {
        offset: Vec3::new(265.0, 0.0, 295.0),
        ptr: cube1,
    });
    /*cube1 = Arc::new(ConstantMedium {
        density: 0.01,
        boundary: cube1,
        phase_function: Arc::new(Isotropic {
            albedo: Arc::new(ConstantTexture {
                color: Vec3::new(0.0, 0.0, 0.0),
            }),
        }),
    });*/
    hitlist.push(cube1);

    /*let mut cube2: Arc<dyn Hitable> = Arc::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    cube2 = Arc::new(RotateY::new(cube2, -18.0));
    cube2 = Arc::new(Translate {
        offset: Vec3::new(130.0, 0.0, 65.0),
        ptr: cube2,
    });
    hitlist.push(cube2);*/
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(190.0, 90.0, 190.0),
        radius: 90.0,
        mat_ptr: glass, //Arc::new(Dielectric { ref_idx: 1.5 }),
    }));

    World::new(hitlist)
}
/*fn final_scene() -> World {
    let light = Arc::new(DiffuseLight {
        emit: Arc::new(ConstantTexture {
            color: Vec3::new(15.0, 15.0, 15.0),
        }),
    });
    let ground = Arc::new(Lambertian {
        albedo: Arc::new(ConstantTexture {
            color: Vec3::new(0.48, 0.83, 0.53),
        }),
    });
    let mut hitlist: Vec<Arc<dyn Hitable>> = vec![Arc::new(XzRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        mat_ptr: light,
    })];

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            hitlist.push(Arc::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        mat_ptr: Arc::new(Metal {
            albedo: Arc::new(ConstantTexture {
                color: Vec3::new(0.8, 0.8, 0.9),
            }),
            fuzzy: 10.0,
        }),
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 0.1,
            }),
        }),
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new()),
        }),
    }));
    hitlist.push(Arc::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new()),
        }),
    }));
    let mut cube1: Arc<dyn Hitable> = Arc::new(Sphere {
        center: Vec3::new(400.0, 400.0, 200.0),
        radius: 50.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: Arc::new(ConstantTexture {
                color: Vec3::new(0.8, 0.2, 0.9),
            }),
        }),
    });
    cube1 = Arc::new(ConstantMedium {
        density: 0.01,
        boundary: cube1,
        phase_function: Arc::new(Isotropic {
            albedo: Arc::new(ConstantTexture {
                color: Vec3::new(0.2, 0.4, 0.9),
            }),
        }),
    });
    hitlist.push(cube1);

    World::new(hitlist)
}
fn scene() -> World {
    let hitlist: Vec<Arc<dyn Hitable>> = vec![
        Arc::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat_ptr: Arc::new(Lambertian {
                albedo: Arc::new(NoiseTexture {
                    noise: Perlin::new(),
                    scale: 4.0,
                }),
            }),
        }),
        Arc::new(Sphere {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            mat_ptr: Arc::new(Lambertian {
                albedo: Arc::new(NoiseTexture {
                    noise: Perlin::new(),
                    scale: 4.0,
                }),
            }),
        }),
    ];
    World::new(hitlist)
}*/

fn work(cam: Camera, wor: World) {
    let mut img: RgbImage = ImageBuffer::new(cam.width, cam.height);
    let bar = ProgressBar::new(cam.width as u64);

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
                    //let co = ray_color(ra, &wor, MAX_DEPTH);
                    let co = ray_color(ra, &wor, MAX_DEPTH)
                        .min(Vec3::ones())
                        .max(Vec3::zero());
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
    let wor = cornell_box();
    /*let cam = Camera::new(
        40f64.to_radians(),
        1.0,
        WIDTH,
        Vec3::new(478.0, 278.0, -600.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    let wor = final_scene();*/
    /*let cam = Camera::new(
        20f64.to_radians(),
        1.0,
        WIDTH,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    let wor = scene();*/
    work(cam, wor);
}
