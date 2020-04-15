use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::EPSILON;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn build(self) -> Primitive {
        Primitive::Sphere(self)
    }
}

impl Hit for Sphere {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let oc = r.orig - self.center;
        let a = r.dir.dot_self();
        let b = oc.dot(&r.dir);
        let c = oc.dot_self() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > EPSILON {
            let temp = (- b - (b.powi(2) - a * c).sqrt()) / a;
            if EPSILON < temp && temp < rec.t {
                rec.active = true;
                rec.t = temp;
                rec.pos = r.project(temp);
                rec.normal = (rec.pos - self.center) / self.radius;
                rec.texture = self.texture;
            }
            let temp = (- b + (b.powi(2) - a * c).sqrt()) / a;
            if EPSILON < temp && temp < rec.t {
                rec.active = true;
                rec.t = temp;
                rec.pos = r.project(temp);
                rec.normal = (rec.pos - self.center) / self.radius;
                rec.texture = self.texture;
            }
        }
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
        Primitive::InfinitePlane(self)
    }
}

impl Hit for InfinitePlane {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let a = r.orig;
        let b = r.dir;
        let bn = b.dot(&self.normal);
        if bn.abs() < EPSILON {
            return;
        } else {
            let temp = - (a - self.orig).dot(&self.normal) / bn;
            if EPSILON < temp && temp < rec.t {
                rec.active = true;
                rec.t = temp;
                rec.pos = r.project(temp);
                rec.normal = self.normal.unit();
                rec.texture = self.texture;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle{
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub texture: Texture,
}

impl Triangle {
    pub fn build(self) -> Primitive {
        Primitive::Triangle(self)
    }
}


impl Hit for Triangle {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let det = -self.u.cross(&self.v).dot(&r.dir);
        let w = r.orig - self.a;
        let a = -w.cross(&self.v).dot(&r.dir) / det;
        let b = -self.u.cross(&w).dot(&r.dir) / det;
        let temp = self.u.cross(&self.v).dot(&w) / det;
        if a > 0. && b > 0. && a + b < 1. && EPSILON < temp && temp < rec.t {
            rec.active = true;
            rec.t = temp;
            rec.pos = r.project(temp);
            rec.normal = self.u.cross(&self.v).unit();
            rec.texture = self.texture;
        }
    }
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
        Primitive::Parallelogram(self)
    }
}

impl Hit for Parallelogram {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let det = -self.u.cross(&self.v).dot(&r.dir);
        let w = r.orig - self.a;
        let a = -w.cross(&self.v).dot(&r.dir) / det;
        let b = -self.u.cross(&w).dot(&r.dir) / det;
        let temp = self.u.cross(&self.v).dot(&w) / det;
        if a > 0. && b > 0. && a < 1. && b < 1. && EPSILON < temp && temp < rec.t {
            rec.active = true;
            rec.t = temp;
            rec.pos = r.project(temp);
            rec.normal = self.u.cross(&self.v).unit();
            rec.texture = self.texture;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rhombus {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct RhombusObject([Parallelogram; 6]);

impl Rhombus {
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
        };//
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
        };//
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
        };//
        Primitive::Rhombus(RhombusObject([s1, s2, s3, s4, s5, s6]))
    }
}

impl Hit for RhombusObject {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        for obj in &self.0 {
            obj.hit(rec, r);
        }
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
        Primitive::EmptyCylinder(self)
    }
}


impl Hit for EmptyCylinder {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let ab = self.center2 - self.center1;
        let ao = r.orig - self.center1;
        let aoxab = ao.cross(&ab);
        let vxab = r.dir.cross(&ab);
        let ab2 = ab.dot_self();
        let a = vxab.dot_self();
        let b = 2.0 * vxab.dot(&aoxab);
        let c = aoxab.dot_self() - self.radius.powi(2) * ab2;

        let det = b.powi(2) - 4.0 * a * c;
        if det < EPSILON {
            return;
        }
        let temp = -(b + det.sqrt()) / (2.0 * a);
        if EPSILON < temp && temp < rec.t {
            let pos = r.project(temp);
            let u = pos - self.center1;
            let udir = ab.unit();
            let maxlen = ab.len();
            let p = u.dot(&udir);
            if 0.0 < p && p < maxlen {
                let v = (u - udir * u.dot(&udir)).unit();
                rec.active = true;
                rec.t = temp;
                rec.pos = pos;
                rec.normal = v.unit();
                rec.texture = self.texture;
            }
        }
    }
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
        Primitive::Disc(self)
    }
}

impl Hit for Disc {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        let a = r.orig;
        let b = r.dir;
        let bn = b.dot(&self.normal);
        if bn.abs() < EPSILON {
            return;
        } else {
            let temp = - (a - self.center).dot(&self.normal) / bn;
            if EPSILON < temp && temp < rec.t {
                let pos = r.project(temp);
                let dist = (pos - self.center).len();
                if dist < self.radius {
                    rec.active = true;
                    rec.t = temp;
                    rec.pos = pos;
                    rec.normal = self.normal.unit();
                    rec.texture = self.texture;
                }
            }
        }
    }
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
        Primitive::Cylinder(CylinderObject {
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
        })
    }
}

impl Hit for CylinderObject {
    fn hit(&self, rec: &mut HitRecord, r: &Ray) {
        self.side.hit(rec, r);
        self.cap1.hit(rec, r);
        self.cap2.hit(rec, r);
    }
}
