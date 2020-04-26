use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub idx: f64,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir, idx: 1.0 }
    }

    pub fn project(&self, idx: f64) -> Vec3 {
        self.orig + self.dir * idx
    }
}
