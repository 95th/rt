use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Vec3, outward_normal: Vec3, t: f64) -> Self {
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
        }
    }
}

pub trait HitTarget {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut last_hit = None;
        for target in self.list.iter() {
            if let Some(hit) = target.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit.t;
                last_hit.replace(hit);
            }
        }
        last_hit
    }
}
