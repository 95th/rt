use crate::{camera::Camera, hit::HitWorld, sphere::Sphere, vec3::Vec3};

mod camera;
mod color;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut world = HitWorld::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(image_width, aspect_ratio);
    camera.render(&world);
}
