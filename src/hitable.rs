use crate::vec3::Vec3;
use crate::ray::Ray;
//use rand::Rng;
use crate::rgb::RGB;
use crate::primitives::*;
use crate::composite_molecules::*;
use crate::composite_cradle::*;
use crate::composite_axes::*;
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

#[derive(Clone)]
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

impl Hit for Primitive {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        match self {
            Primitive::Sphere(s) => s.hit(rec, r),
            Primitive::InfinitePlane(s) => s.hit(rec, r),
            Primitive::Triangle(s) => s.hit(rec, r),
            Primitive::Parallelogram(s) => s.hit(rec, r),
            Primitive::Rhombus(s) => s.hit(rec, r),
            Primitive::EmptyCylinder(s) => s.hit(rec, r),
            Primitive::Disc(s) => s.hit(rec, r),
            Primitive::Cylinder(s) => s.hit(rec, r),
        }
    }
}

pub type Composite = Vec<Primitive>;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub active: bool,
    pub t: f64,
    pub pos: Vec3,
    pub normal: Vec3,
    pub texture: Texture,
}

pub trait Hit {
    fn hit(&self, rec: &mut HitRecord, r: &Ray);
}

#[derive(Clone)]
pub struct World(Composite);

impl World {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, x: Primitive) {
        self.0.push(x);
    }

    pub fn push_vec(&mut self, v: Composite) {
        for x in v {
            self.0.push(x)
        }
    }
}


impl World {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        for obj in &self.0 {
            obj.hit(rec, r)
        }
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

pub fn scatter(incident: &Ray, record: &HitRecord) -> Option<(RGB, Ray)> {
    match record.texture {
        Texture::Lambertian(albedo) => {
            let target = record.pos + record.normal + random_in_unit_sphere() * 0.8;
            let scattered = Ray::new(record.pos, target - record.pos);
            let attenuation = albedo;
            Some((attenuation, scattered))
        }
        Texture::Metal(albedo, fuzziness) => {
            let reflec = incident.dir.unit().reflect(&record.normal);
            let scattered = Ray::new(record.pos, reflec + random_in_unit_sphere() * fuzziness);
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
                    (-record.normal, idx, idx * incident.dir.dot(&record.normal) / incident.dir.len())
                } else {
                    (record.normal, 1.0 / idx, -incident.dir.dot(&record.normal) / incident.dir.len())
                }
            };
            match incident.dir.refract(&ext_normal, rel_idx) {
                None => Some((shade, Ray { orig: record.pos, dir: reflected })),
                Some(refracted) => {
                    let prob_reflect = schlick(cos, idx);
                    if rand::random::<f64>() < prob_reflect {
                        Some((shade, Ray { orig: record.pos, dir: reflected }))
                    } else {
                        Some((shade, Ray { orig: record.pos, dir: refracted }))
                    }
                }
            }
        }
    }
}


pub fn color(r: &Ray, w: &World, depth: i32) -> RGB {
    let mut record = HitRecord {
        active: false,
        t: 1000.,
        pos: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        texture: Texture::Lambertian(RGB::new(0.0, 0.0, 0.0)),
    };
    w.hit(&mut record, r);
    if record.active {
        if depth < 100 {
            if let Some((attenuation, scattered)) = scatter(r, &record) {
                attenuation * color(&scattered, &w, depth+1)
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
    } else {
        let u = r.dir.unit();
        let t = 0.5 * (u.y + 1.);
        RGB::new(0.9, 0.9, 0.9) * (1. - t) + RGB::new(0.5, 0.7, 1.) * t
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
