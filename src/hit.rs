use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        point: Vec3,
        outward_normal: Vec3,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait HitTarget {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HitWorld {
    list: Vec<Box<dyn HitTarget>>,
}

impl HitWorld {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push(&mut self, target: impl HitTarget + 'static) {
        self.list.push(Box::new(target));
    }
}

impl HitTarget for HitWorld {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut last_hit = None;
        for target in self.list.iter() {
            if let Some(hit) = target.hit(ray, ray_t.with_max(closest_so_far)) {
                closest_so_far = hit.t;
                last_hit.replace(hit);
            }
        }
        last_hit
    }
}
