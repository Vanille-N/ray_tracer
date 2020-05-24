use crate::internal::*;
use std::sync::Arc;

macro_rules! auto_texture {
    () => {
        fn texture(&self) -> Texture {
            self.texture
        }
    };
}

macro_rules! empty_object {
    () => {
        fn inside(&self, _pos: Vec3) -> bool {
            false
        }
    };
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray) -> HitRecord {
        let oc = r.orig - self.center;
        let a = r.dir.dot_self();
        let b = oc.dot(r.dir);
        let c = oc.dot_self() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        let mut rec = HitRecord::Blank;
        if discriminant > EPSILON {
            let temp = (-b - (b.powi(2) - a * c).sqrt()) / a;
            if EPSILON < temp {
                let pos = r.project(temp);
                rec.compare(HitRecord::make(temp, pos, pos - self.center, self.texture))
            }
            let temp = (-b + (b.powi(2) - a * c).sqrt()) / a;
            if EPSILON < temp {
                let pos = r.project(temp);
                rec.compare(HitRecord::make(temp, pos, pos - self.center, self.texture))
            }
        }
        rec
    }

    auto_texture! {}

    fn inside(&self, pos: Vec3) -> bool {
        (pos - self.center).len() < self.radius
    }
}

#[derive(Clone, Copy)]
pub struct InfinitePlane {
    pub orig: Vec3,
    pub normal: Vec3,
    pub texture: Texture,
}

impl InfinitePlane {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for InfinitePlane {
    fn hit(&self, r: &Ray) -> HitRecord {
        let a = r.orig;
        let b = r.dir;
        let bn = b.dot(self.normal);
        if bn.abs() > EPSILON {
            let temp = -(a - self.orig).dot(self.normal) / bn;
            if EPSILON < temp {
                HitRecord::make(temp, r.project(temp), self.normal, self.texture)
            } else {
                HitRecord::Blank
            }
        } else {
            HitRecord::Blank
        }
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub texture: Texture,
}

impl Triangle {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for Triangle {
    fn hit(&self, r: &Ray) -> HitRecord {
        let det = -self.u.cross(self.v).dot(r.dir);
        let w = r.orig - self.a;
        let a = -w.cross(self.v).dot(r.dir) / det;
        let b = -self.u.cross(w).dot(r.dir) / det;
        let temp = self.u.cross(self.v).dot(w) / det;
        if a > 0. && b > 0. && a + b < 1. && EPSILON < temp {
            HitRecord::make(temp, r.project(temp), self.u.cross(self.v), self.texture)
        } else {
            HitRecord::Blank
        }
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Clone, Copy)]
pub struct Parallelogram {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub texture: Texture,
}

impl Parallelogram {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for Parallelogram {
    fn hit(&self, r: &Ray) -> HitRecord {
        let det = -self.u.cross(self.v).dot(r.dir);
        let w = r.orig - self.a;
        let a = -w.cross(self.v).dot(r.dir) / det;
        let b = -self.u.cross(w).dot(r.dir) / det;
        let temp = self.u.cross(self.v).dot(w) / det;
        if a > 0. && b > 0. && a < 1. && b < 1. && EPSILON < temp {
            HitRecord::make(temp, r.project(temp), self.u.cross(self.v), self.texture)
        } else {
            HitRecord::Blank
        }
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Clone, Copy)]
pub struct Rhomboid {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct RhomboidObject(pub [Parallelogram; 6]);

impl Rhomboid {
    pub fn orthogonal(self) -> Self {
        let wlen = self.w.len();
        let ulen = self.u.len();
        let vlen = self.v.len();
        let w = self.w.unit() * wlen; // Upwards
        let u = self.v.cross(w).unit() * ulen;
        let v = w.cross(u).unit() * vlen;
        Self {
            a: self.a,
            u,
            v,
            w,
            texture: self.texture,
        }
    }

    pub fn orthonormal(self) -> Self {
        let len = self.w.len();
        let w = self.w.unit() * len; // Upwards
        let u = self.v.cross(w).unit() * len;
        let v = w.cross(u).unit() * len;
        Self {
            a: self.a,
            u,
            v,
            w,
            texture: self.texture,
        }
    }

    pub fn build(self) -> Primitive {
        let s1 = Parallelogram {
            a: self.a,
            u: self.u,
            v: self.v,
            texture: self.texture,
        };
        let s2 = Parallelogram {
            a: self.a,
            u: self.v,
            v: self.w,
            texture: self.texture,
        }; //
        let s3 = Parallelogram {
            a: self.a,
            u: self.w,
            v: self.u,
            texture: self.texture,
        };
        let s4 = Parallelogram {
            a: self.a + self.v,
            u: self.w,
            v: self.u,
            texture: self.texture,
        }; //
        let s5 = Parallelogram {
            a: self.a + self.w,
            u: self.u,
            v: self.v,
            texture: self.texture,
        };
        let s6 = Parallelogram {
            a: self.a + self.u,
            u: self.v,
            v: self.w,
            texture: self.texture,
        }; //
        Primitive(Arc::new(RhomboidObject([s1, s2, s3, s4, s5, s6])))
    }
}

impl Hit for RhomboidObject {
    fn hit(&self, r: &Ray) -> HitRecord {
        let mut rec = HitRecord::Blank;
        for obj in &self.0 {
            rec.compare(obj.hit(r));
        }
        rec
    }

    fn texture(&self) -> Texture {
        self.0[0].texture
    }

    fn inside(&self, pos: Vec3) -> bool {
        Interaction::bidir_hit(self, pos, Vec3(0.0, 1.0, 0.0))
    }
}

#[derive(Clone, Copy)]
pub struct EmptyCylinder {
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

impl EmptyCylinder {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for EmptyCylinder {
    fn hit(&self, ray: &Ray) -> HitRecord {
        let ab = self.center2 - self.center1;
        let ao = ray.orig - self.center1;
        let aoxab = ao.cross(ab);
        let vxab = ray.dir.cross(ab);
        let ab2 = ab.dot_self();
        let a = vxab.dot_self();
        let b = 2.0 * vxab.dot(aoxab);
        let c = aoxab.dot_self() - self.radius.powi(2) * ab2;

        let det = b.powi(2) - 4.0 * a * c;
        if det < EPSILON {
            return HitRecord::Blank;
        }
        let mut rec = HitRecord::Blank;
        let temp = -(b + det.sqrt()) / (2.0 * a);
        if EPSILON < temp {
            let pos = ray.project(temp);
            let u = pos - self.center1;
            let udir = ab.unit();
            let maxlen = ab.len();
            let proj = u.dot(udir);
            if 0.0 < proj && proj < maxlen {
                let normal = (u - udir * u.dot(udir)).unit();
                rec.compare(HitRecord::make(temp, pos, normal, self.texture));
            }
        }
        let temp = -(b - det.sqrt()) / (2.0 * a);
        if EPSILON < temp {
            let pos = ray.project(temp);
            let u = pos - self.center1;
            let udir = ab.unit();
            let maxlen = ab.len();
            let proj = u.dot(udir);
            if 0.0 < proj && proj < maxlen {
                let normal = (u - udir * u.dot(udir)).unit();
                rec.compare(HitRecord::make(temp, pos, normal, self.texture));
            }
        }
        rec
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Clone, Copy)]
pub struct Disc {
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

impl Disc {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for Disc {
    fn hit(&self, r: &Ray) -> HitRecord {
        let a = r.orig;
        let b = r.dir;
        let bn = b.dot(self.normal);
        if bn.abs() > EPSILON {
            let temp = -(a - self.center).dot(self.normal) / bn;
            if EPSILON < temp {
                let pos = r.project(temp);
                let dist = (pos - self.center).len();
                if dist < self.radius {
                    return HitRecord::make(temp, pos, self.normal, self.texture);
                }
            }
        }
        HitRecord::Blank
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Clone, Copy)]
pub struct Cylinder {
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct CylinderObject {
    pub side: EmptyCylinder,
    pub cap1: Disc,
    pub cap2: Disc,
}

impl Cylinder {
    pub fn build(self) -> Primitive {
        let n = (self.center2 - self.center1).unit();
        Primitive(Arc::new(CylinderObject {
            side: EmptyCylinder {
                center1: self.center1,
                center2: self.center2,
                radius: self.radius,
                texture: self.texture,
            },
            cap1: Disc {
                center: self.center1,
                radius: self.radius,
                normal: n,
                texture: self.texture,
            },
            cap2: Disc {
                center: self.center2,
                radius: self.radius,
                normal: -n,
                texture: self.texture,
            },
        }))
    }
}

impl Hit for CylinderObject {
    fn hit(&self, r: &Ray) -> HitRecord {
        let mut rec = HitRecord::Blank;
        rec.compare(self.side.hit(r));
        rec.compare(self.cap1.hit(r));
        rec.compare(self.cap2.hit(r));
        rec
    }

    fn texture(&self) -> Texture {
        self.side.texture
    }

    fn inside(&self, pos: Vec3) -> bool {
        Interaction::bidir_hit(self, pos, self.cap1.normal)
    }
}

#[derive(Clone, Copy)]
pub struct EmptyCone {
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
}

impl EmptyCone {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(self))
    }
}

impl Hit for EmptyCone {
    fn hit(&self, ray: &Ray) -> HitRecord {
        // let a = (ray.dir.dot(&self.dir)).powi(2) - self.angle.cos().powi(2);
        // let b = 2. * (ray.dir.dot(&self.dir) * (ray.orig - self.orig).dot(&self.dir) - ray.dir.dot(&(ray.orig - self.orig)) * self.angle.cos().powi(2));
        // let c = (ray.orig - self.orig).dot(&self.dir).powi(2) - (ray.orig - self.orig).dot_self() * self.angle.cos().powi(2);

        let axis = self.dir;
        let theta = axis.unit();
        let tan2 = self.angle.tan().powi(2);
        let ap = ray.orig;
        let ad = ray.dir;
        let diff = ap - self.orig;
        let a = ad.dot_self() - (tan2 + 1.) * ad.dot(theta).powi(2);
        let b = 2. * (ad.dot(diff) - (tan2 + 1.) * ad.dot(theta) * diff.dot(theta));
        let c = diff.dot_self() - (tan2 + 1.) * diff.dot(theta).powi(2);

        let det = b.powi(2) - 4.0 * a * c;
        if det < EPSILON {
            return HitRecord::Blank;
        }
        let mut rec = HitRecord::Blank;
        let temp = -(b + det.sqrt()) / (2.0 * a);
        if EPSILON < temp {
            let pos = ray.project(temp);
            let u = pos - self.orig;
            let proj = u.dot(self.dir.unit());
            if self.begin < proj && proj < self.end {
                let tangent = u.cross(self.dir);
                let normal = u.cross(tangent);
                rec.compare(HitRecord::make(temp, pos, normal, self.texture));
            }
        }
        let temp = -(b - det.sqrt()) / (2.0 * a);
        if EPSILON < temp {
            let pos = ray.project(temp);
            let u = pos - self.orig;
            let proj = u.dot(self.dir.unit());
            if self.begin < proj && proj < self.end {
                let tangent = u.cross(self.dir);
                let normal = u.cross(tangent);
                rec.compare(HitRecord::make(temp, pos, normal, self.texture));
            }
        }
        rec
    }

    auto_texture! {}
    empty_object! {}
}

#[derive(Copy, Clone)]
pub struct Cone {
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
}

#[derive(Copy, Clone)]
pub struct ConeObject {
    pub side: EmptyCone,
    pub cap1: Disc,
    pub cap2: Disc,
}

impl Cone {
    pub fn build(self) -> Primitive {
        Primitive(Arc::new(ConeObject {
            side: EmptyCone {
                orig: self.orig,
                dir: self.dir,
                angle: self.angle,
                begin: self.begin,
                end: self.end,
                texture: self.texture,
            },
            cap1: Disc {
                center: self.orig + self.dir.unit() * self.begin,
                radius: self.begin * self.angle.tan(),
                normal: -self.dir.unit(),
                texture: self.texture,
            },
            cap2: Disc {
                center: self.orig + self.dir.unit() * self.end,
                radius: self.end * self.angle.tan(),
                normal: self.dir.unit(),
                texture: self.texture,
            },
        }))
    }
}

impl Hit for ConeObject {
    fn hit(&self, r: &Ray) -> HitRecord {
        let mut rec = HitRecord::Blank;
        rec.compare(self.side.hit(r));
        rec.compare(self.cap1.hit(r));
        rec.compare(self.cap2.hit(r));
        rec
    }

    fn texture(&self) -> Texture {
        self.side.texture
    }

    fn inside(&self, pos: Vec3) -> bool {
        let u = (pos - self.side.orig).unit();
        let v = u - self.side.dir * u.dot(self.side.dir);
        Interaction::bidir_hit(self, pos, v.cross(self.side.dir))
    }
}
