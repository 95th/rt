use crate::{
    hit::{Hit, HitRecord},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if t <= ray_tmin || t >= ray_tmax {
            t = (b + sqrtd) / a;
            if t <= ray_tmin || t >= ray_tmax {
                return None;
            }
        }

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord { point, normal, t })
    }
}
