use crate::{color::Color, hit::HitTarget, interval::Interval, ray::Ray, vec3::Vec3};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let image_height = image_width as f64 / aspect_ratio;
        let image_height = if image_height >= 1.0 {
            image_height as u32
        } else {
            1
        };
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::default();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, target: &dyn HitTarget) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&ray, target);
                println!("{}", color.to_int());
            }
        }
        eprintln!("Done");
    }

    fn ray_color(&self, ray: &Ray, target: &dyn HitTarget) -> Color {
        if let Some(hit) = target.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            let c = 0.5 * (hit.normal + 1.0);
            return Color::from(c);
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        let c = (1.0 - a) * Vec3::splat(1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        Color::from(c)
    }
}
