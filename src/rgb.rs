use std::ops;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGB { r, g, b }
    }

    pub fn shade(self, p: u8) -> Self {
        self * p as f64 / 100.0
    }

    pub fn mix(self, other: Self) -> Self {
        (self + other) / 2.0
    }
}

impl ops::Add for RGB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::AddAssign for RGB {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Mul<RGB> for RGB {
    type Output = Self;

    fn mul(self, other: RGB) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl ops::MulAssign<RGB> for RGB {
    fn mul_assign(&mut self, other: RGB) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl ops::Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl ops::MulAssign<f64> for RGB {
    fn mul_assign(&mut self, other: f64) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}

impl ops::Sub for RGB {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl ops::SubAssign for RGB {
    fn sub_assign(&mut self, other: Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl ops::Div<RGB> for RGB {
    type Output = Self;

    fn div(self, other: RGB) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl ops::DivAssign<RGB> for RGB {
    fn div_assign(&mut self, other: RGB) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
    }
}

impl ops::Div<f64> for RGB {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl ops::DivAssign<f64> for RGB {
    fn div_assign(&mut self, other: f64) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.r < 0. || self.g < 0. || self.b < 0. {
            panic!("Trying to output invalid color: {} {} {}", self.r, self.g, self.b);
        }
        let r = (min(self.r.sqrt(), 1.0) * 255.99) as u8;
        let g = (min(self.g.sqrt(), 1.0) * 255.99) as u8;
        let b = (min(self.b.sqrt(), 1.0) * 255.99) as u8;
        write!(f, "{} {} {} ", r, g, b)
    }
}
