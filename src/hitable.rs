use crate::primitives::*;
use crate::ray::Ray;
use crate::rgb::RGB;
use crate::vec3::Vec3;
use crate::EPSILON;

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
    EmptyCone(EmptyCone),
    Cone(ConeObject),
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
pub struct Interaction(Vec<Primitive>, Vec<Primitive>);

impl Interaction {
    pub fn bidir_hit(obj: &Primitive, pos: Vec3, v: Vec3) -> bool {
        let ray1 = Ray {
            orig: pos,
            dir: v,
        };
        let ray2 = Ray {
            orig: pos,
            dir: -v,
        };
        match (obj.hit(&ray1), obj.hit(&ray2)) {
            (HitRecord::Blank, _) => false,
            (_, HitRecord::Blank) => false,
            (_, _) => true,
        }
    }

    pub fn inside(obj: &Primitive, pos: Vec3) -> bool {
        match *obj {
            Primitive::Sphere(s) => {
                (pos - s.center).len() < s.radius
            }
            Primitive::InfinitePlane(_) => false,
            Primitive::Triangle(_) => false,
            Primitive::Parallelogram(_) => false,
            Primitive::Rhombus(_) => {
                Interaction::bidir_hit(obj, pos, Vec3::new(0.0, 1.0, 0.0))
            }
            Primitive::EmptyCylinder(_) => false,
            Primitive::Disc(_) => false,
            Primitive::Cylinder(s) => {
                Interaction::bidir_hit(obj, pos, s.cap1.normal)
            }
            Primitive::EmptyCone(_) => false,
            Primitive::Cone(s) => {
                let u = (pos - s.side.orig).unit();
                let v = u - s.side.dir * u.dot(&s.side.dir);
                Interaction::bidir_hit(obj, pos, v.cross(&s.side.dir))
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

    pub fn caracteristics(&self, pos: Vec3) -> (f64, RGB) {
        for group in &self.0 {
            if Interaction::all_inside_except(pos, &group.0, group.0.len())
            && Interaction::all_outside_except(pos, &group.1, group.1.len())
                {
                for item in &group.0 {
                    match item.texture() {
                        Texture::Dielectric(shade, idx) => return (idx, shade),
                        _ => (),
                    }
                }
            }
        }
        return (1., RGB::new(1., 1., 1.));
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
fn schlick(cos: f64, n1: f64, n2: f64) -> f64 {
    let r = ((n1 - n2) / (n1 + n2)).powi(2);
    r + (1.0 - r) * (1.0 - cos).powi(5)
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn scatter(incident: &Ray, record: ActiveHit, w: &World) -> Option<(RGB, Ray)> {
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
        Texture::Dielectric(shade, _idx) => {
            let reflected = incident.dir.reflect(&record.normal).unit();
            let ext_normal = {
                if incident.dir.dot(&record.normal) > 0.0 {
                    -record.normal
                } else {
                    record.normal
                }
            };
            let tmp_ray_succ = Ray { orig: record.pos, dir: ext_normal };
            let tmp_ray_prev = Ray { orig: record.pos, dir: -ext_normal };
            let mid_caract = |r| {
                match w.hit(&r) {
                    HitRecord::Blank => (1., RGB::new(1., 1., 1.), 1.),
                    HitRecord::Hit(h) => {
                        let mid = (h.pos + record.pos) / 2.;
                        let (idx, shade) = w.caracteristics(mid);
                        let len = (h.pos - record.pos).len();
                        (idx, shade, len)
                    }
                }
            };
            let (i_idx, i_shade, i_len) = mid_caract(tmp_ray_prev);
            let (r_idx, _, _) = mid_caract(tmp_ray_succ);
            let rel_idx = r_idx / i_idx;
            let cos = -incident.dir.unit().dot(&ext_normal);


            match incident.dir.refract(&ext_normal, rel_idx) {
                None => Some((
                    shade,
                    Ray {
                        orig: record.pos,
                        dir: reflected,
                    },
                )),
                Some(refracted) => {
                    let prob_reflect = schlick(cos, i_idx, r_idx);
                    if rand::random::<f64>() < prob_reflect {
                        Some((
                            shade,
                            Ray {
                                orig: record.pos,
                                dir: reflected,
                            },
                        ))
                    } else {
                        let shade = RGB::new(1., 1., 1.) - (RGB::new(1., 1., 1.) - i_shade) * i_len * 1.5;

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

pub fn color(r: &Ray, w: &World, depth: i32, sky: &Sky) -> RGB {
    match w.hit(r) {
        HitRecord::Hit(record) => {
            if depth < 100 {
                if let Some((attenuation, scattered)) = scatter(r, record, w) {
                    attenuation * color(&scattered, &w, depth + 1, sky)
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
            sky.color(r.dir)
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

#[derive(Clone)]
pub struct Sky {
    map: Vec<Vec<RGB>>,
    hgt: usize,
    wth: usize,
}

impl Sky {
    pub fn new(file: &str) -> Self {
        let s = std::fs::read_to_string(file)
            .unwrap()
            .replace("\n", " ")
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut it = s.iter();
        let _ = it.next();
        let mut get = || { it.next().unwrap().parse::<usize>().unwrap() };
        let wth = get();
        let hgt = get();
        let max = get() as f64;
        let mut map = Vec::new();
        for _ in 0..hgt {
            let mut v = Vec::new();
            for _ in 0..wth {
                let r = get() as f64;
                let g = get() as f64;
                let b = get() as f64;
                v.push(RGB::new(r / max, g / max, b / max));
            }
            map.push(v);
        }
        Self {
            map,
            hgt,
            wth,
        }
    }

    pub fn color(&self, v: Vec3) -> RGB {
        let (x, y) = {
            let mut v = v;
            v.y = 0.;
            let v = v.unit();
            (v.x, v.z)
        };
        let rise = v.unit().y.abs();
        let mid_i = self.hgt as f64 / 2.;
        let mid_j = self.wth as f64 / 2.;
        let rad = mid_i.min(mid_j) / 1.1;
        let i = mid_i + y * rad * (1. - rise);
        let j = mid_j + x * rad * (1. - rise);
        //println!("{} {}", i, j);
        self.map[i as usize][j as usize]
        //RGB::new((x+1.)/2., (y+1.)/2., (rise+1.)/2.)
    }
}
