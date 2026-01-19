use rand::{Rng, rng};

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

pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray.direction.unit();

        let cos_theta = (unit_dir * -1.0).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > rng().random() {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, ri)
        };

        let scattered = Ray::new(hit.point, direction);
        let attenuation = Vec3::splat(1.0);
        Some(Scatter {
            attenuation,
            scattered,
        })
    }
}
