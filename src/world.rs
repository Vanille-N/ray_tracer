use crate::hitable::{Interaction, Composite, HitRecord, Texture, Hit};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::rgb::RGB;
use crate::EPSILON;

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
    pub fn hit(&self, r: &Ray) -> HitRecord {
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
                    if let Texture::Dielectric(shade, idx) = item.texture() {
                        return (idx, shade);
                    }
                }
            }
        }
        (1., RGB(1., 1., 1.))
    }
}
