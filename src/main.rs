use std::path::Path;
use std::{fs::File, io::BufWriter};

extern crate glam;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod utils;
use glam::Vec3;
use hitable::Hitable;
use ray::Ray;

use crate::camera::Camera;
use crate::material::{Lambertian, Metal};
use crate::utils::random_double;
use crate::{hitable::HitableList, sphere::Sphere};

use self::utils::random_in_hemisphere;

fn main() {
    let width: u16 = 1200;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let height: u16 = (width as f32 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, u32::from(width), u32::from(height));
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let material_ground = Box::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material = Box::new(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });
    let material_metal = Box::new(Metal {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });

    let world = HitableList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(-0.5, 0.0, -1.0),
                radius: 0.5,
                material,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.5, 0.0, -1.0),
                radius: 0.5,
                material: material_metal,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                material: material_ground,
                radius: 100.0,
            }),
        ],
    };

    let camera = Camera::new();

    let mut pixels: Vec<u8> = vec![];

    for y in (0..=height - 1).rev() {
        for x in 0..width {
            let mut color = Vec3::ZERO;
            for s in 0..samples_per_pixel {
                let u: f32 = ((x as f32) + random_double()) / ((width as f32) - 1.0);
                let v: f32 = ((y as f32) + random_double()) / ((height as f32) - 1.0);

                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, max_depth);
            }
            let scale = 1.0 / (samples_per_pixel as f32);
            let r = (color.x * scale).sqrt();
            let g = (color.y * scale).sqrt();
            let b = (color.z * scale).sqrt();

            pixels.push((256.0 * r.clamp(0.0, 0.999)) as u8);
            pixels.push((256.0 * g.clamp(0.0, 0.999)) as u8);
            pixels.push((256.0 * b.clamp(0.0, 0.999)) as u8);
            pixels.push(255);
        }
    }
    writer.write_image_data(&pixels).unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Hitable, depth: u8) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {

        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return scatter.attenuation * ray_color(&scatter.scattered, world, depth -1);
        }

        return Vec3::ZERO;
    }

    let unit_direction = ray.direction.normalize();

    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}
