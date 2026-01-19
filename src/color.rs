use std::fmt;

use crate::{interval::Interval, vec3::Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Color<T = f64> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl Color<f64> {
    pub fn to_int(self) -> Color<u8> {
        let Self { r, g, b } = self;
        let [r, g, b] = [r, g, b].map(|c| {
            const INTENSITY: Interval = Interval::new(0.0, 0.999);
            let corrected = linear_to_gamma(c);
            (256.0 * INTENSITY.clamp(corrected)) as u8
        });
        Color::new(r, g, b)
    }
}

impl From<Vec3> for Color {
    fn from(Vec3 { x, y, z }: Vec3) -> Self {
        Self::new(x, y, z)
    }
}

impl<T: fmt::Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b } = self;
        write!(f, "{r} {g} {b}")
    }
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}
