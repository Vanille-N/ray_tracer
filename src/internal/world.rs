use std::sync::Arc;
use crate::internal::*;

#[derive(Clone, Default)]
pub struct World {
    obj: Vec<Interaction>,
    pub background: Option<RGB>,
}

impl World {
    pub fn new() -> Self {
        Self {
            obj: Vec::new(),
            background: None,
        }
    }

    pub fn push(&mut self, x: Interaction) {
        self.obj.push(x);
    }

    pub fn push_vec(&mut self, v: Composite) {
        for x in v {
            self.obj.push(x)
        }
    }

    pub fn hit(&self, r: &Ray) -> HitRecord {
        let mut rec = HitRecord::Blank;
        for group in &self.obj {
            let mut record = HitRecord::Blank;
            for i in 0..group.0.len() {
                let mut ray = *r;
                let mut offset = 0.0;
                let item = &group.0[i];
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
                let item = &group.1[i];
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
        for group in &self.obj {
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

//  https://en.wikipedia.org/wiki/Schlick's_approximation
fn schlick(cos: f64, n1: f64, n2: f64) -> f64 {
    let r = ((n1 - n2) / (n1 + n2)).powi(2);
    r + (1.0 - r) * (1.0 - cos).powi(5)
}

pub fn scatter(incident: &Ray, record: ActiveHit, w: &World) -> Option<(RGB, Ray)> {
    match record.texture {
        Texture::Lambertian(albedo) => {
            let reflec = incident.dir.unit().reflect(record.normal);
            let scattered = Ray::new(record.pos, reflec + Vec3::random_unit() * 0.8);
            let attenuation = albedo;
            let normal = {
                if scattered.dir.dot(record.normal) > 0.0 {
                    record.normal
                } else {
                    -record.normal
                }
            };
            if scattered.dir.dot(normal) > EPSILON {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
        Texture::Metal(albedo, fuzziness) => {
            let reflec = incident.dir.unit().reflect(record.normal);
            let scattered = Ray::new(record.pos, reflec + Vec3::random_unit() * fuzziness * 0.8);
            let attenuation = albedo;
            let normal = {
                if scattered.dir.dot(record.normal) > 0.0 {
                    record.normal
                } else {
                    -record.normal
                }
            };
            if scattered.dir.dot(normal) > EPSILON {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
        Texture::Light(_) => None,
        Texture::Dielectric(shade, _idx) => {
            let reflected = incident.dir.reflect(record.normal).unit();
            let ext_normal = {
                if incident.dir.dot(record.normal) > 0.0 {
                    -record.normal
                } else {
                    record.normal
                }
            };
            let tmp_ray_succ = Ray {
                orig: record.pos,
                dir: ext_normal,
            };
            let tmp_ray_prev = Ray {
                orig: record.pos,
                dir: -ext_normal,
            };
            let mid_caract = |r| match w.hit(&r) {
                HitRecord::Blank => (1., RGB(1., 1., 1.), 1.),
                HitRecord::Hit(h) => {
                    let mid = (h.pos + record.pos) / 2.;
                    let (idx, shade) = w.caracteristics(mid);
                    let len = (h.pos - record.pos).len();
                    (idx, shade, len)
                }
            };
            let (i_idx, i_shade, i_len) = mid_caract(tmp_ray_prev);
            let (r_idx, _, _) = mid_caract(tmp_ray_succ);
            let rel_idx = r_idx / i_idx;
            let cos = -incident.dir.unit().dot(ext_normal);

            match incident.dir.refract(ext_normal, rel_idx) {
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
                        let shade = RGB(1., 1., 1.) - (RGB(1., 1., 1.) - i_shade) * i_len * 1.5;

                        Some((
                            shade.validate(),
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
        HitRecord::Blank => sky.color(r.dir),
    }
}

pub fn calc_color(r: &Ray, w: &World, sky: &Sky) -> RGB {
    match w.hit(r) {
        HitRecord::Hit(record) => {
            if let Some((attenuation, scattered)) = scatter(r, record, w) {
                attenuation * color(&scattered, &w, 1, sky)
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
            match w.background {
                None => sky.color(r.dir),
                Some(c) => c,
            }
        }
    }
}
