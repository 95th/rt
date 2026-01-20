use std::rc::Rc;

use rand::{Rng, rng};

use crate::{
    camera::Camera,
    hit::HitWorld,
    material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial},
    sphere::Sphere,
    vec3::Vec3,
};

mod camera;
mod color;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HitWorld::new();
    let ground_material = Rc::new(LambertianMaterial::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    Rc::new(LambertianMaterial::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Rc::new(MetalMaterial::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(DielectricMaterial::new(1.5))
                };
                world.push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Rc::new(DielectricMaterial::new(1.5));
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Rc::new(LambertianMaterial::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Rc::new(MetalMaterial::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    camera.render(&world);
}
