use crate::primitives::*;
use crate::ray::Ray;
use crate::rgb::RGB;
use crate::vec3::Vec3;
use crate::sky::Sky;
use crate::world::World;
use crate::EPSILON;

pub trait Hit {
    fn hit(&self, r: &Ray) -> HitRecord;
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub enum Primitive {
    Sphere(Sphere),
    InfinitePlane(InfinitePlane),
    Triangle(Triangle),
    Parallelogram(Parallelogram),
    Rhombus(RhombusObject),
    EmptyCylinder(EmptyCylinder),
    Disc(Disc),
    Cylinder(CylinderObject),
    EmptyCone(EmptyCone),
    Cone(ConeObject),
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
        match self {
            Primitive::Sphere(s) => s.texture,
            Primitive::InfinitePlane(s) => s.texture,
            Primitive::Triangle(s) => s.texture,
            Primitive::Parallelogram(s) => s.texture,
            Primitive::Rhombus(s) => s.0[0].texture,
            Primitive::EmptyCylinder(s) => s.texture,
            Primitive::Disc(s) => s.texture,
            Primitive::Cylinder(s) => s.side.texture,
            Primitive::EmptyCone(s) => s.texture,
            Primitive::Cone(s) => s.side.texture,
        }
    }
}

impl Hit for Primitive {
    fn hit(&self, r: &Ray) -> HitRecord {
        match self {
            Primitive::Sphere(s) => s.hit(r),
            Primitive::InfinitePlane(s) => s.hit(r),
            Primitive::Triangle(s) => s.hit(r),
            Primitive::Parallelogram(s) => s.hit(r),
            Primitive::Rhombus(s) => s.hit(r),
            Primitive::EmptyCylinder(s) => s.hit(r),
            Primitive::Disc(s) => s.hit(r),
            Primitive::Cylinder(s) => s.hit(r),
            Primitive::EmptyCone(s) => s.hit(r),
            Primitive::Cone(s) => s.hit(r),
        }
    }
}

#[derive(Clone)]
pub struct Interaction(pub Vec<Primitive>, pub Vec<Primitive>);

impl Interaction {
    pub fn bidir_hit(obj: &Primitive, pos: Vec3, v: Vec3) -> bool {
        let ray1 = Ray { orig: pos, dir: v };
        let ray2 = Ray { orig: pos, dir: -v };
        match (obj.hit(&ray1), obj.hit(&ray2)) {
            (HitRecord::Blank, _) => false,
            (_, HitRecord::Blank) => false,
            (_, _) => true,
        }
    }

    pub fn inside(obj: &Primitive, pos: Vec3) -> bool {
        match *obj {
            Primitive::Sphere(s) => (pos - s.center).len() < s.radius,
            Primitive::InfinitePlane(_) => false,
            Primitive::Triangle(_) => false,
            Primitive::Parallelogram(_) => false,
            Primitive::Rhombus(_) => Interaction::bidir_hit(obj, pos, Vec3(0.0, 1.0, 0.0)),
            Primitive::EmptyCylinder(_) => false,
            Primitive::Disc(_) => false,
            Primitive::Cylinder(s) => Interaction::bidir_hit(obj, pos, s.cap1.normal),
            Primitive::EmptyCone(_) => false,
            Primitive::Cone(s) => {
                let u = (pos - s.side.orig).unit();
                let v = u - s.side.dir * u.dot(s.side.dir);
                Interaction::bidir_hit(obj, pos, v.cross(s.side.dir))
            }
        }
    }

    pub fn outside(obj: &Primitive, pos: Vec3) -> bool {
        !Interaction::inside(obj, pos)
    }

    pub fn intersect(&mut self, other: Primitive) {
        self.0.push(other);
    }

    pub fn remove(&mut self, other: Primitive) {
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
