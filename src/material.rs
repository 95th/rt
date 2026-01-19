use crate::{hit::HitRecord, ray::Ray, vec3::Vec3};

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

pub struct LambertianMaterial {
    albedo: Vec3,
}

impl LambertianMaterial {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let mut scatter_dir = hit.normal + Vec3::random_unit();
        if scatter_dir.is_near_zero() {
            scatter_dir = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_dir);
        let attenuation = self.albedo;
        Some(Scatter {
            attenuation,
            scattered,
        })
    }
}

pub struct MetalMaterial {
    albedo: Vec3,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.reflect(hit.normal);
        let reflected = reflected.unit() + self.fuzz * Vec3::random_unit();
        let scattered = Ray::new(hit.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some(Scatter {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
