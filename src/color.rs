use std::fmt::{self, Display};

pub struct Color<T = f64> {
    r: T,
    g: T,
    b: T,
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

impl<T: Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b } = self;
        writeln!(f, "{r} {g} {b}")
    }
}
