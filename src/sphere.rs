use std::rc::Rc;

use crate::{
    hit::{HitRecord, HitTarget},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl HitTarget for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.len_squared();
        let b = ray.direction.dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;
        let d = b * b - a * c;
        if d < 0.0 {
            return None;
        }

        let sqrtd = d.sqrt();
        let mut t = (b - sqrtd) / a;
        if !ray_t.surrounds(t) {
            t = (b + sqrtd) / a;
            if !ray_t.surrounds(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(
            ray,
            point,
            outward_normal,
            t,
            self.material.clone(),
        ))
    }
}
