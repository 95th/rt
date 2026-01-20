use std::rc::Rc;

use crate::{
    camera::Camera,
    hit::HitWorld,
    material::{DielectricMaterial, LambertianMaterial, MetalMaterial},
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
    let mat_ground = Rc::new(LambertianMaterial::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(LambertianMaterial::new(Vec3::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(DielectricMaterial::new(1.5));
    let mat_bubble = Rc::new(DielectricMaterial::new(1.0 / 1.5));
    let mat_right = Rc::new(MetalMaterial::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HitWorld::new();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_center));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
    );
    camera.render(&world);
}
