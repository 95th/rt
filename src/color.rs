use std::fmt;

use crate::vec3::Vec3;

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
        Color {
            r: (255.999 * self.r) as u8,
            g: (255.999 * self.g) as u8,
            b: (255.999 * self.b) as u8,
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self {
            r: value.x,
            g: value.y,
            b: value.z,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b } = self;
        writeln!(f, "{r} {g} {b}")
    }
}
