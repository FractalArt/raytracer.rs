use rand::prelude::*;
use std::rc::Rc;

use raytracer::camera::Camera;
use raytracer::materials::Dielectric;
use raytracer::materials::Lambertian;
use raytracer::materials::Metal;
use raytracer::objects::sphere::Sphere;
use raytracer::objects::Hitable;
use raytracer::objects::HitableList;
use raytracer::ray::Ray;
use raytracer::vec3::*;

// For now the color is a simple gradient background.
fn color(r: &Ray, world: &Box<dyn Hitable>, depth: i32) -> Vec3 {
    match world.intersect(&r, 0.001, std::f32::MAX) {
        Some(hit) => {
            let scatter_info = hit.material.scatter(&r, &hit);
            match scatter_info {
                Some((scattered, attenuation)) => {
                    if depth < 50 {
                        attenuation * color(&scattered, world, depth + 1)
                    } else {
                        Default::default()
                    }
                }
                None => Default::default(),
            }
        }
        None => {
            let unit_direction = unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> Box<dyn Hitable> {
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        Rc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
    )));

    // Random number generator
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + 0.6 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.6 * rng.gen::<f32>(),
            );
            let c1 = Vec3(4.0, 1.0, 0.);
            let c2 = Vec3(-4.0, 1.0, 0.);
            let c3 = Vec3(0.0, 1.0, 0.);
            if (center - c1).length() > 1.2 && (center - c2).length() > 1.2 && (center - c3).length() > 1.2  {
                if choose_mat < 0.8 {
                    // diffuse
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Vec3(
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3(
                                0.5 * (1. + rng.gen::<f32>()),
                                0.5 * (1. + rng.gen::<f32>()),
                                0.5 * (1. + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    )));
                } else {
                    // glass
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3(0., 1., 0.),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3(-4., 1., 0.),
        1.0,
        Rc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3(4., 1., 0.),
        1.0,
        Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));

    Box::new(HitableList::new(list))
}

fn main() {
    println!("Raytracer in Rust!");

    let nx = 1200;
    let ny = 800;
    let ns = 150;

    let mut img: image::RgbImage = image::ImageBuffer::new(nx, ny);

    // Create the world.
    let mut object_list: Vec<Box<dyn Hitable>> = vec![];

    object_list.push(Box::new(Sphere::new(
        Vec3(0., 0., -1.),
        0.5,
        Rc::new(Lambertian::new(Vec3(0.8, 0.3, 0.3))),
    )));
    object_list.push(Box::new(Sphere::new(
        Vec3(0., -100.5, -1.),
        100.0,
        Rc::new(Lambertian::new(Vec3(0.8, 0.8, 0.))),
    )));
    object_list.push(Box::new(Sphere::new(
        Vec3(1., 0., -1.),
        0.5,
        Rc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.03)),
    )));
    object_list.push(Box::new(Sphere::new(
        Vec3(-1., 0., -1.),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    object_list.push(Box::new(Sphere::new(
        Vec3(-1., 0., -1.),
        -0.45,
        Rc::new(Dielectric::new(1.5)),
    )));
    let world = random_scene();
    // Set up the camera
    let look_from = Vec3(13., 2., 3.);
    let look_at = Vec3(0., 0., 0.);
    let aperture = 0.1; // 0.1;
    let dist_to_focus = 10.; // 10;
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        20.,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    // Random number generator
    let mut rng = rand::thread_rng();

    // Generate the image.
    (0..ny).into_iter().for_each(|y| {
        (0..nx).into_iter().for_each(|x| {
            let y = ny - y - 1;

            let mut col = Vec3(0., 0., 0.);
            for _ in 0..ns {
                let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (y as f32 + rng.gen::<f32>()) as f32 / ny as f32;

                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            let r = (col.r().sqrt() * 254.99) as u8;
            let g = (col.g().sqrt() * 254.99) as u8;
            let b = (col.b().sqrt() * 254.99) as u8;
            let color = image::Rgb([r, g, b]);
            img.put_pixel(x, ny - y - 1, color);
        })
    });

    img.save("image.png").unwrap();
}
