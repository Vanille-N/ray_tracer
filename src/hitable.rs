use crate::primitives::*;
use crate::ray::Ray;
use crate::rgb::RGB;
use crate::vec3::Vec3;
use crate::EPSILON;

// Warning:
// To get a working multithreading, and because I was tired of cryptic errors:

// error[E0277]:`std::ptr::NonNull<rand::rngs::adapter::ReseedingRng<
// rand_chacha::chacha::ChaCha20Core, rand::rngs::OsRng>>` cannot be
// sent between threads safely
//    --> src/main.rs:142:28
//     |
// 142 |     writers.par_iter_mut().for_each(|(f, range)| {
//     |                            ^^^^^^^^ `std::ptr::NonNull<rand::rngs::ada
// pter::ReseedingRng<rand_chacha::chacha::ChaCha20Core, rand::rngs::OsRng>>` cannot
// be sent between threads safely
//     |
//     = help: within `rand::prelude::ThreadRng`, the trait `std::marker::Send`
// is not implemented for `std::ptr::NonNull<rand::rngs::adapter::Reseedin
// gRng<rand_chacha::chacha::ChaCha20Core, rand::rngs::OsRng>>`
//    = note: required because it appears within the type `rand::prelude::ThreadRng`
//    = note: required because of the requirements on the impl of `std::marker::
// Send` for `&mut rand::prelude::ThreadRng`
//    = note: required because it appears within the type `[closure@src
// /main.rs:142:37: 163:6 ni:&i32, nj:&i32, ns:&i32, rng:&mut
//  rand::prelude::ThreadRng, cam:&camera::Camera, w:&hitable::World<'_>]`

// I settled for some horrible design decisions.
// Maybe this will improve in the future, when I become more comfortable with traits.
// The good news is that this seems to have negligible effect on performance,
// and it is barely visible from the outside of this file.
// From the point of view of someone creating objects, it does require implementing
// the `build` method on all objects. Since complex objects already require it, this
// can be considered a minimal hassle.
// Altogether it seems to be a small price to pay for access to multithreading and
// relief from manually managing lifetimes.

#[derive(Clone, Copy)]
pub enum Primitive {
    Sphere(Sphere),
    InfinitePlane(InfinitePlane),
    Triangle(Triangle),
    Parallelogram(Parallelogram),
    Rhombus(RhombusObject),
    EmptyCylinder(EmptyCylinder),
    Disc(Disc),
    Cylinder(CylinderObject),
}

impl Primitive {
    pub fn wrap(self) -> Interaction {
        Interaction(vec![self], vec![])
    }

    pub fn intersect(self, other: Primitive) -> Interaction {
        Interaction(vec![self, other], vec![])
    }

    pub fn remove(self, other: Primitive) -> Interaction {
        Interaction(vec![self], vec![other])
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
        }
    }
}

#[derive(Clone)]
pub struct Interaction(Vec<Primitive>, Vec<Primitive>);

impl Interaction {
    pub fn inside(obj: &Primitive, pos: Vec3) -> bool {
        let ray1 = Ray {
            orig: pos,
            dir: Vec3::new(0.0, 1.0, 0.0),
        };
        let ray2 = Ray {
            orig: pos,
            dir: Vec3::new(0.0, -1.0, 0.0),
        };
        match (obj.hit(&ray1), obj.hit(&ray2)) {
            (HitRecord::Blank, _) => false,
            (_, HitRecord::Blank) => false,
            (_, _) => true,
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

#[derive(Copy, Clone, Debug)]
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

pub trait Hit {
    fn hit(&self, r: &Ray) -> HitRecord;
}

#[derive(Clone)]
pub struct World(Composite);

impl World {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, x: Interaction) {
        self.0.push(x);
    }

    pub fn push_vec(&mut self, v: Composite) {
        for x in v {
            self.0.push(x)
        }
    }
}

impl World {
    fn hit(&self, r: &Ray) -> HitRecord {
        let mut rec = HitRecord::Blank;
        for group in &self.0 {
            let mut record = HitRecord::Blank;
            for i in 0..group.0.len() {
                let mut ray = *r;
                let mut offset = 0.0;
                let item = group.0[i];
                loop {
                    match item.hit(&ray) {
                        HitRecord::Blank => break,
                        HitRecord::Hit(h) => {
                            if Interaction::all_inside_except(h.pos, &group.0, i)
                                && Interaction::all_outside_except(h.pos, &group.1, group.1.len())
                            {
                                record.compare(HitRecord::Hit(h.later(offset)));
                            }
                            ray.orig = h.pos + ray.dir * EPSILON;
                            offset += h.t;
                        }
                    }
                }
            }
            for i in 0..group.1.len() {
                let mut ray = *r;
                let mut offset = 0.0;
                let item = group.1[i];
                loop {
                    match item.hit(&ray) {
                        HitRecord::Blank => break,
                        HitRecord::Hit(h) => {
                            if Interaction::all_inside_except(h.pos, &group.0, group.0.len())
                                && Interaction::all_outside_except(h.pos, &group.1, i)
                            {
                                record.compare(HitRecord::Hit(h.later(offset)));
                            }
                            ray.orig = h.pos + ray.dir * EPSILON;
                            offset += h.t;
                        }
                    }
                }
            }
            rec.compare(record);
        }
        rec
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Texture {
    Lambertian(RGB),
    Metal(RGB, f64),
    Light(RGB),
    Dielectric(RGB, f64),
}

//  https://en.wikipedia.org/wiki/Schlick's_approximation
fn schlick(cos: f64, idx: f64) -> f64 {
    let r = ((1.0 - idx) / (1.0 + idx)).powi(2);
    r + (1.0 - r) * (1.0 - cos).powi(5)
}

pub fn scatter(incident: &Ray, record: ActiveHit) -> Option<(RGB, Ray)> {
    match record.texture {
        Texture::Lambertian(albedo) => {
            let reflec = incident.dir.unit().reflect(&record.normal);
            let scattered = Ray::new(record.pos, reflec + random_in_unit_sphere() * 0.8);
            let attenuation = albedo;
            let normal = {
                if scattered.dir.dot(&record.normal) > 0.0 {
                    record.normal
                } else {
                    -record.normal
                }
            };
            if scattered.dir.dot(&normal) > EPSILON {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
        Texture::Metal(albedo, fuzziness) => {
            let reflec = incident.dir.unit().reflect(&record.normal);
            let scattered = Ray::new(
                record.pos,
                reflec + random_in_unit_sphere() * fuzziness * 0.8,
            );
            let attenuation = albedo;
            let normal = {
                if scattered.dir.dot(&record.normal) > 0.0 {
                    record.normal
                } else {
                    -record.normal
                }
            };
            if scattered.dir.dot(&normal) > EPSILON {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
        Texture::Light(_) => None,
        Texture::Dielectric(shade, idx) => {
            let reflected = incident.dir.reflect(&record.normal);
            let (ext_normal, rel_idx, cos) = {
                if incident.dir.dot(&record.normal) > 0.0 {
                    (
                        -record.normal,
                        idx,
                        idx * incident.dir.dot(&record.normal) / incident.dir.len(),
                    )
                } else {
                    (
                        record.normal,
                        1.0 / idx,
                        -incident.dir.dot(&record.normal) / incident.dir.len(),
                    )
                }
            };
            match incident.dir.refract(&ext_normal, rel_idx) {
                None => Some((
                    shade,
                    Ray {
                        orig: record.pos,
                        dir: reflected,
                    },
                )),
                Some(refracted) => {
                    let prob_reflect = schlick(cos, idx);
                    if rand::random::<f64>() < prob_reflect {
                        Some((
                            shade,
                            Ray {
                                orig: record.pos,
                                dir: reflected,
                            },
                        ))
                    } else {
                        Some((
                            shade,
                            Ray {
                                orig: record.pos,
                                dir: refracted,
                            },
                        ))
                    }
                }
            }
        }
    }
}

pub fn color(r: &Ray, w: &World, depth: i32) -> RGB {
    match w.hit(r) {
        HitRecord::Hit(record) => {
            if depth < 100 {
                if let Some((attenuation, scattered)) = scatter(r, record) {
                    attenuation * color(&scattered, &w, depth + 1)
                } else {
                    match record.texture {
                        Texture::Lambertian(color) => color,
                        Texture::Metal(color, _) => color,
                        Texture::Light(color) => color,
                        Texture::Dielectric(color, _) => color,
                    }
                }
            } else {
                match record.texture {
                    Texture::Lambertian(color) => color,
                    Texture::Metal(color, _) => color,
                    Texture::Light(color) => color,
                    Texture::Dielectric(color, _) => color,
                }
            }
        }
        HitRecord::Blank => {
            let u = r.dir.unit();
            let t = 0.5 * (u.y + 1.);
            RGB::new(0.9, 0.9, 0.9) * (1. - t) + RGB::new(0.5, 0.7, 1.) * t
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.square_len() >= 1. {
        p.x = rand::random::<f64>() * 2. - 1.;
        p.y = rand::random::<f64>() * 2. - 1.;
        p.z = rand::random::<f64>() * 2. - 1.;
    }
    p
}
