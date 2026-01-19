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
        const INTENSITY: Interval = Interval::new(0.0, 0.999);
        let Color { r, g, b } = self;
        Color::new(
            (256.0 * INTENSITY.clamp(r)) as u8,
            (256.0 * INTENSITY.clamp(g)) as u8,
            (256.0 * INTENSITY.clamp(b)) as u8,
        )
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
