use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct RGB(
    pub f64,
    pub f64,
    pub f64,
);

impl ops::Add for RGB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for RGB {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::Mul<RGB> for RGB {
    type Output = Self;

    fn mul(self, other: RGB) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign<RGB> for RGB {
    fn mul_assign(&mut self, other: RGB) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl ops::Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::MulAssign<f64> for RGB {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::Sub for RGB {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign for RGB {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl ops::Div<RGB> for RGB {
    type Output = Self;

    fn div(self, other: RGB) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl ops::DivAssign<RGB> for RGB {
    fn div_assign(&mut self, other: RGB) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl ops::Div<f64> for RGB {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl ops::DivAssign<f64> for RGB {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 0. || self.1 < 0. || self.2 < 0. {
            panic!(
                "Trying to output invalid color: {} {} {}",
                self.0, self.1, self.2
            );
        }
        let r = (self.0.sqrt().min(1.0) * 255.99) as u8;
        let g = (self.1.sqrt().min(1.0) * 255.99) as u8;
        let b = (self.2.sqrt().min(1.0) * 255.99) as u8;
        write!(f, "{} {} {} ", r, g, b)
    }
}

impl ops::Rem<usize> for RGB {
    type Output = Self;
    fn rem(self, part: usize) -> Self::Output {
        self * part as f64 / 100.
    }
}

pub const RED: RGB = RGB(1.0, 0.0, 0.0);
pub const DKRED: RGB = RGB(0.5, 0.0, 0.0);
pub const LTRED: RGB = RGB(1.0, 0.5, 0.5);
pub const BLUE: RGB = RGB(0.0, 0.0, 1.0);
pub const DKBLUE: RGB = RGB(0.0, 0.0, 0.5);
pub const LTBLUE: RGB = RGB(0.3, 0.6, 1.0);
pub const CYAN: RGB = RGB(0.0, 1.0, 1.0);
pub const GREEN: RGB = RGB(0.0, 1.0, 0.0);
pub const DKGREEN: RGB = RGB(0.0, 0.5, 0.0);
pub const LTGREEN: RGB = RGB(0.7, 1.0, 0.0);
pub const PURPLE: RGB = RGB(0.7, 0.0, 0.0);
pub const MAGENTA: RGB = RGB(1.0, 0.0, 1.0);
pub const YELLOW: RGB = RGB(1.0, 1.0, 0.0);
pub const BROWN: RGB = RGB(0.3, 0.2, 0.0);
pub const ORANGE: RGB = RGB(1.0, 0.4, 0.0);
pub const TURQUOISE: RGB = RGB(0.0, 0.9, 0.6);
pub const BLACK: RGB = RGB(0.0, 0.0, 0.0);
pub const WHITE: RGB = RGB(1.0, 1.0, 1.0);
pub const GREY: RGB = RGB(0.5, 0.5, 0.5);
pub const DKGREY: RGB = RGB(0.2, 0.2, 0.2);
pub const LTGREY: RGB = RGB(0.8, 0.8, 0.8);
