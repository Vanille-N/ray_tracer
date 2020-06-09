use std::ops;

use crate::internal::EPSILON;

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

#[allow(clippy::len_without_is_empty)]
impl Vec3 {
    pub fn len(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    /// Unit vector with same direction
    pub fn unit(&self) -> Self {
        let len = self.len();
        Self(self.0 / len, self.1 / len, self.2 / len)
    }

    /// Dot product with another vector
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Square of the length
    pub fn dot_self(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Cross product with another vector
    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0,
        )
    }

    /// Symetry from the surface normal
    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * self.dot(normal) * 2.
    }

    /// Refracted ray from Snell and Descartes' law
    pub fn refract(&self, normal: Self, rel_idx: f64) -> Option<Self> {
        let u = self.unit();
        let dt = u.dot(normal);
        let discriminant = 1.0 - rel_idx.powi(2) * (1.0 - dt.powi(2));
        if discriminant > EPSILON {
            Some((u - normal * dt) * rel_idx - normal * discriminant.sqrt())
        } else {
            None
        }
    }

    /// Random ray in the unit sphere.
    ///
    /// Expected to fail about half of the time, meaning that the inner loop
    /// should run an average of two times per function call.
    ///
    /// See [this blog post](http://datagenetics.com/blog/january32020/index.html)
    /// for a discussion on the subject
    pub fn random_unit() -> Self {
        let mut p = Self(1.0, 1.0, 1.0);
        while p.dot_self() >= 1. {
            p.0 = rand::random::<f64>() * 2. - 1.;
            p.1 = rand::random::<f64>() * 2. - 1.;
            p.2 = rand::random::<f64>() * 2. - 1.;
        }
        p
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
