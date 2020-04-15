use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::rgb::RGB;
use crate::primitives::*;

const RED: Texture = Texture::Lambertian(RGB { r: 0.9, g: 0.1, b: 0.1 });
const BLUE: Texture = Texture::Lambertian(RGB { r: 0.0, g: 0.2, b: 0.7 });
const GREEN: Texture = Texture::Lambertian(RGB { r: 0.0, g: 0.9, b: 0.0 });
const BLACK: Texture = Texture::Lambertian(RGB { r: 0.01, g: 0.01, b: 0.01 });

#[derive(Clone, Copy)]
pub struct Axes(pub f64);

#[derive(Clone)]
pub struct AxesObject {
    pub lines: Vec<EmptyCylinder>,
    pub balls: Vec<Sphere>,
}

#[allow(unused_variables)]
impl Axes {
    pub fn build(self) -> Composite {
        let len = self.0;
        let rad1 = len * 0.1;
        let rad2 = len * 0.2;
        let o = Vec3::new(0.0, 0.0, 0.0);
        let x = Vec3::new(1.0, 0.0, 0.0) * len;
        let y = Vec3::new(0.0, 1.0, 0.0) * len;
        let z = Vec3::new(0.0, 0.0, 1.0) * len;
        let orig = Sphere {
            center: o,
            radius: rad2,
            texture: BLACK,
        };
        let xdir = EmptyCylinder {
            center1: o,
            center2: o + x * 5.0,
            radius: rad1,
            texture: RED,
        };
        let ydir = EmptyCylinder {
            center1: o,
            center2: o + y * 5.0,
            radius: rad1,
            texture: BLUE,
        };
        let zdir = EmptyCylinder {
            center1: o,
            center2: o + z * 5.0,
            radius: rad1,
            texture: GREEN,
        };
        let mut v = vec![orig];
        for ix in 1..=5 {
            v.push(Sphere {
                center: o + x * ix as f64,
                radius: rad2,
                texture: RED,
            });
        }
        for iy in 1..=5 {
            v.push(Sphere {
                center: o + y * iy as f64,
                radius: rad2,
                texture: BLUE,
            });
        }
        for iz in 1..=5 {
            v.push(Sphere {
                center: o + z * iz as f64,
                radius: rad2,
                texture: GREEN,
            });
        }
        Composite::Axes(AxesObject { lines: vec![xdir, ydir, zdir], balls: v })
    }
}

impl Hit for AxesObject {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t.max;
        for obj in &self.lines {
            match obj.hit(r, Interval { max: closest, ..t }) {
                None => (),
                Some(rec) => {
                    closest = rec.t;
                    record = Some(rec);
                }
            }
        }
        for obj in &self.balls {
            match obj.hit(r, Interval { max: closest, ..t }) {
                None => (),
                Some(rec) => {
                    closest = rec.t;
                    record = Some(rec);
                }
            }
        }
        record
    }
}
