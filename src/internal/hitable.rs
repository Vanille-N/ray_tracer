use std::sync::Arc;
use crate::internal::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray) -> HitRecord;
    fn texture(&self) -> Texture;
    fn inside(&self, pos: Vec3) -> bool;
}

#[derive(Clone, Copy)]
pub struct ActiveHit {
    pub t: f64,
    pub pos: Vec3,
    pub normal: Vec3,
    pub texture: Texture,
}

impl ActiveHit {
    pub fn later(self, t: f64) -> Self {
        ActiveHit {
            t: self.t + t,
            ..self
        }
    }
}

pub enum HitRecord {
    Blank,
    Hit(ActiveHit),
}

impl HitRecord {
    pub fn make(t: f64, pos: Vec3, normal: Vec3, texture: Texture) -> Self {
        HitRecord::Hit(ActiveHit {
            t,
            pos,
            normal: normal.unit(),
            texture,
        })
    }

    pub fn compare(&mut self, other: Self) {
        match other {
            HitRecord::Blank => (),
            HitRecord::Hit(b) => match self {
                HitRecord::Blank => *self = other,
                HitRecord::Hit(a) => {
                    if a.t > b.t {
                        *self = other;
                    }
                }
            },
        }
    }
}

pub struct Primitive(pub Arc<dyn Hit>);

impl Clone for Primitive {
    fn clone(&self) -> Self {
        Primitive(self.0.clone())
    }
}

impl Primitive {
    pub fn wrap(self) -> Interaction {
        Interaction(vec![self], vec![])
    }

    pub fn intersect(self, other: Self) -> Interaction {
        Interaction(vec![self, other], vec![])
    }

    pub fn remove(self, other: Self) -> Interaction {
        Interaction(vec![self], vec![other])
    }

    pub fn texture(&self) -> Texture {
        self.0.texture()
    }

    pub fn hit(&self, r: &Ray) -> HitRecord {
        self.0.hit(r)
    }

    pub fn inside(&self, pos: Vec3) -> bool {
        self.0.inside(pos)
    }
}

#[derive(Clone)]
pub struct Interaction(pub Vec<Primitive>, pub Vec<Primitive>);

impl Interaction {
    pub fn bidir_hit<T: Hit>(obj: &T, pos: Vec3, v: Vec3) -> bool {
        let ray1 = Ray { orig: pos, dir: v };
        let ray2 = Ray { orig: pos, dir: -v };
        match (obj.hit(&ray1), obj.hit(&ray2)) {
            (HitRecord::Blank, _) => false,
            (_, HitRecord::Blank) => false,
            (_, _) => true,
        }
    }

    pub fn inside(obj: &Primitive, pos: Vec3) -> bool {
        obj.inside(pos)
    }

    pub fn outside(obj: &Primitive, pos: Vec3) -> bool {
        !Interaction::inside(obj, pos)
    }

    pub fn intersect(mut self, other: Primitive) -> Self {
        self.0.push(other);
        self
    }

    pub fn remove(mut self, other: Primitive) -> Self {
        self.1.push(other);
        self
    }

    pub fn intersect_mut(&mut self, other: Primitive) {
        self.0.push(other);
    }

    pub fn remove_mut(&mut self, other: Primitive) {
        self.1.push(other);
    }

    pub fn all_inside_except(p: Vec3, v: &[Primitive], i: usize) -> bool {
        for (j, item) in v.iter().enumerate() {
            if j != i && Interaction::outside(item, p) {
                return false;
            }
        }
        true
    }

    pub fn all_outside_except(p: Vec3, v: &[Primitive], i: usize) -> bool {
        for (j, item) in v.iter().enumerate() {
            if j != i && Interaction::inside(item, p) {
                return false;
            }
        }
        true
    }
}

pub type Composite = Vec<Interaction>;

#[derive(Clone, Copy)]
pub enum Texture {
    Lambertian(RGB),
    Metal(RGB, f64),
    Light(RGB),
    Dielectric(RGB, f64),
}
