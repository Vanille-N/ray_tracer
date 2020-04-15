use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::rgb::RGB;
use crate::primitives::*;

#[derive(Clone, Copy)]
pub struct NewtonCraddle {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

#[derive(Clone)]
pub struct NewtonCraddleObject {
    stand: [Primitive; 11],
    beads: [Primitive; 10],
    threads: [Primitive; 10],
}

impl NewtonCraddle {
    pub fn build(self) -> Composite {
        let plastic = Texture::Lambertian(RGB::new(0.1, 0.1, 0.1));
        let steel = Texture::Metal(RGB::new(0.8, 0.6, 0.2), 0.0);
        let nylon = Texture::Lambertian(RGB::new(0.9, 0.9, 0.9));
        let len = self.w.len();
        let w = self.w.unit() * len; // Upwards
        let u = self.v.cross(&w).unit() * len;
        let v = w.cross(&u).unit() * len;
        let pedestal = Rhombus {
            a: self.a, u: u, v: v, w: w * 0.1,
            texture: plastic,
        }.build();
        let pillar1 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.1,
            center2: self.a + v * 0.1 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let pillar2 = EmptyCylinder {
            center1: self.a + v * 0.9 + u * 0.1,
            center2: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let pillar3 = EmptyCylinder {
            center1: self.a + v * 0.9 + u * 0.9,
            center2: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let pillar4 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.9,
            center2: self.a + v * 0.1 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let bar1 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.1 + w,
            center2: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let bar2 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.9 + w,
            center2: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let smoothtop1 = Sphere {
            center: self.a + v * 0.1 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let smoothtop2 = Sphere {
            center: self.a + v * 0.1 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let smoothtop3 = Sphere {
            center: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let smoothtop4 = Sphere {
            center: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }.build();
        let r = u.len() * 0.7 / 10.0;
        let sphere1 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring1 = Sphere {
            center: sphere1.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread1a = EmptyCylinder {
            center1: ring1.center,
            center2: ring1.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let thread1b = EmptyCylinder {
            center1: ring1.center,
            center2: ring1.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let sphere2 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 3. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring2 = Sphere {
            center: sphere2.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread2a = EmptyCylinder {
            center1: ring2.center,
            center2: ring2.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let thread2b = EmptyCylinder {
            center1: ring2.center,
            center2: ring2.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let sphere3 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 5. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring3 = Sphere {
            center: sphere3.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread3a = EmptyCylinder {
            center1: ring3.center,
            center2: ring3.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let thread3b = EmptyCylinder {
            center1: ring3.center,
            center2: ring3.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let sphere4 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 7. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring4 = Sphere {
            center: sphere4.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread4a = EmptyCylinder {
            center1: ring4.center,
            center2: ring4.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let thread4b = EmptyCylinder {
            center1: ring4.center,
            center2: ring4.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let sphere5 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 9. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring5 = Sphere {
            center: sphere5.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread5a = EmptyCylinder {
            center1: ring5.center,
            center2: ring5.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        let thread5b = EmptyCylinder {
            center1: ring5.center,
            center2: ring5.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }.build();
        Composite::NewtonCraddle(NewtonCraddleObject {
            stand: [pedestal, pillar1, pillar2, pillar3, pillar4, bar1, bar2, smoothtop1, smoothtop2, smoothtop3, smoothtop4],
            beads: [sphere1.build(), sphere2.build(), sphere3.build(), sphere4.build(), sphere5.build(), ring1.build(), ring2.build(), ring3.build(), ring4.build(), ring5.build()],
            threads: [thread1a, thread1b, thread2a, thread2b, thread3a, thread3b, thread4a, thread4b, thread5a, thread5b],
        })
    }
}

impl Hit for NewtonCraddleObject {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t.max;
        for obj in &self.stand {
            match obj.hit(r, Interval { max: closest, ..t }) {
                None => (),
                Some(rec) => {
                    closest = rec.t;
                    record = Some(rec);
                }
            }
        }
        for obj in &self.beads {
            match obj.hit(r, Interval { max: closest, ..t }) {
                None => (),
                Some(rec) => {
                    closest = rec.t;
                    record = Some(rec);
                }
            }
        }
        for obj in &self.threads {
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
