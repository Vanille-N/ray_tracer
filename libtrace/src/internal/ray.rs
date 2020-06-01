use crate::internal::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// When `temp` describes the real numbers, `self.project(temp)`
    /// describes all of the line with the direction of `self.dir`
    pub fn project(&self, temp: f64) -> Vec3 {
        self.orig + self.dir * temp
    }
}
