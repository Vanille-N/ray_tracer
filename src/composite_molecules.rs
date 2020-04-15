use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::rgb::RGB;
use crate::primitives::*;

const BLACK: RGB = RGB { r: 0.05, g: 0.05, b: 0.05 };
const RED: RGB = RGB { r: 0.9, g: 0.1, b: 0.1 };
const BLUE: RGB = RGB { r: 0.0, g: 0.2, b: 0.7 };
const WHITE: RGB = RGB { r: 0.9, g: 0.9, b: 0.9 };
const LGREY: RGB = RGB { r: 0.7, g: 0.7, b: 0.7 };

#[derive(Clone, Copy)]
pub struct Molecule {
    pub c_ref: Vec3,
    pub up: Vec3,
    pub fwd: Vec3,
}

#[derive(Clone)]
pub struct MoleculeObject {
    pub atoms: Vec<Sphere>,
    pub links: Vec<EmptyCylinder>,
}

impl Molecule {
    fn directions(self) -> [Vec3; 7] {
        let zz = self.up.unit();
        let yy = self.fwd.cross(&self.up).unit();
        let xx = zz.cross(&yy).unit();
        // Abstract away link directions
        let x = zz;
        let v =  xx * 0.00 + yy * 0.87 + zz * 0.50;
        let t =  xx * 0.74 - yy * 0.44 + zz * 0.50;
        let u = -xx * 0.74 - yy * 0.44 + zz * 0.50;
        let z =  xx * 0.74 - yy * 0.44 - zz * 0.50;
        let w = -xx * 0.74 - yy * 0.44 - zz * 0.50;
        let y =  xx * 0.00 + yy * 0.87 - zz * 0.50;
        [t, u, v, w, x, y, z]
    }

    pub fn cyclohexanol(self) -> MoleculeObject {
        let len = self.fwd.len();
        let rad1 = len * 1.4;
        let rad2 = len * 0.8;
        let rad3 = len * 0.3;
        let len1 = len * 5.0;
        let len2 = len * 3.0;
        let carbon = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(BLACK, 0.0),
        };
        let oxygen = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(RED, 0.0),
        };
        let nitrogen = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(BLUE, 0.0),
        };
        let hydrogen = |u| Sphere {
            center: u,
            radius: rad2,
            texture: Texture::Metal(WHITE, 0.0),
        };
        let link = |c1, c2| EmptyCylinder {
            center1: c1,
            center2: c2,
            radius: rad3,
            texture: Texture::Lambertian(LGREY),
        };

        let [t, u, v, w, x, y, z] = self.directions();
        // ... And build the molecule skeleton
        let c1 = self.c_ref;
        let c2 = c1 - t * len1;
        let c3 = c1 - u * len1;
        let c4 = c2 + v * len1;
        let c5 = c3 + v * len1;
        let c6 = c4 - u * len1;
        let n = c6 + v * len1;
        let c7 = n + x * len1;
        let o = c1 - v * len1;
        // Then add all hydrogens
        let h1 = o + t * len2;
        let h2 = c1 + x * len2;
        let h3 = c2 - x * len2;
        let h4 = c2 + u * len2;
        let h5 = c3 - x * len2;
        let h6 = c3 + t * len2;
        let h7 = c4 + x * len2;
        let h8 = c4 - t * len2;
        let h9 = c5 + x * len2;
        let h10 = c5 - u * len2;
        let h11 = c6 - x * len2;
        let h12 = n - t * len2;
        let h13 = c7 + t * len2;
        let h14 = c7 + v * len2;
        let h15 = c7 + u * len2;


        MoleculeObject {
            atoms: vec![
                carbon(c1), carbon(c2), carbon(c3), carbon(c4),
                carbon(c5), carbon(c6), carbon(c7),
                oxygen(o), nitrogen(n),
                hydrogen(h1), hydrogen(h2), hydrogen(h3),
                hydrogen(h4), hydrogen(h5), hydrogen(h6),
                hydrogen(h7), hydrogen(h8), hydrogen(h9),
                hydrogen(h10), hydrogen(h11), hydrogen(h12),
                hydrogen(h13), hydrogen(h14), hydrogen(h15),
                ],
            links: vec![
                link(c1, o), link(c1, c2), link(c1, c3), link(c2, c4),
                link(c3, c5), link(c4, c6), link(c5, c6), link(c6, n),
                link(n, c7),
                link(o, h1), link(c1, h2), link(c2, h3), link(c2, h4),
                link(c3, h5), link(c3, h6), link(c4, h7), link(c4, h8),
                link(c5, h9), link(c5, h10), link(c6, h11), link(n, h12),
                link(c7, h13), link(c7, h14), link(c7, h15),
                ],
        }
    }

    pub fn water(self) -> MoleculeObject {
        let len = self.fwd.len();
        let rad1 = len * 1.4;
        let rad2 = len * 0.8;
        let rad3 = len * 0.3;
        let len1 = len * 5.0;
        let len2 = len * 3.0;
        let oxygen = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(RED, 0.0),
        };
        let hydrogen = |u| Sphere {
            center: u,
            radius: rad2,
            texture: Texture::Metal(WHITE, 0.0),
        };
        let link = |c1, c2| EmptyCylinder {
            center1: c1,
            center2: c2,
            radius: rad3,
            texture: Texture::Lambertian(LGREY),
        };

        let [t, u, v, w, x, y, z] = self.directions();
        // ... And build the molecule skeleton
        let o = self.c_ref;
        let h1 = o + x * len2;
        let h2 = o - v * len2;


        MoleculeObject {
            atoms: vec![
                oxygen(o), hydrogen(h1), hydrogen(h2),
                ],
            links: vec![
                link(o, h1), link(o, h2),
                ],
        }
    }

    pub fn methane(self) -> MoleculeObject {
        let len = self.fwd.len();
        let rad1 = len * 1.4;
        let rad2 = len * 0.8;
        let rad3 = len * 0.3;
        let len1 = len * 5.0;
        let len2 = len * 3.0;
        let carbon = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(BLACK, 0.0),
        };
        let hydrogen = |u| Sphere {
            center: u,
            radius: rad2,
            texture: Texture::Metal(WHITE, 0.0),
        };
        let link = |c1, c2| EmptyCylinder {
            center1: c1,
            center2: c2,
            radius: rad3,
            texture: Texture::Lambertian(LGREY),
        };

        let [t, u, v, w, x, y, z] = self.directions();
        // ... And build the molecule skeleton
        let c = self.c_ref;
        let h1 = c + x * len2;
        let h2 = c - u * len2;
        let h3 = c - v * len2;
        let h4 = c - t * len2;


        MoleculeObject {
            atoms: vec![
                carbon(c), hydrogen(h1), hydrogen(h2), hydrogen(h3),
                hydrogen(h4),
                ],
            links: vec![
                link(c, h1), link(c, h2), link(c, h3), link(c, h4),
                ],
        }
    }

    pub fn ethanol(self) -> MoleculeObject {
        let len = self.fwd.len();
        let rad1 = len * 1.4;
        let rad2 = len * 0.8;
        let rad3 = len * 0.3;
        let len1 = len * 5.0;
        let len2 = len * 3.0;
        let carbon = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(BLACK, 0.0),
        };
        let oxygen = |u| Sphere {
            center: u,
            radius: rad1,
            texture: Texture::Metal(RED, 0.0),
        };
        let hydrogen = |u| Sphere {
            center: u,
            radius: rad2,
            texture: Texture::Metal(WHITE, 0.0),
        };
        let link = |c1, c2| EmptyCylinder {
            center1: c1,
            center2: c2,
            radius: rad3,
            texture: Texture::Lambertian(LGREY),
        };

        let [t, u, v, w, x, y, z] = self.directions();
        // ... And build the molecule skeleton
        let c1 = self.c_ref;
        let c2 = c1 - v * len1;
        let o = c1 - t * len1;
        let h1 = c1 + x * len2;
        let h2 = c1 - u * len2;
        let h3 = o + v * len2;
        let h4 = c2 + t * len2;
        let h5 = c2 + u * len2;
        let h6 = c2 - x * len2;


        MoleculeObject {
            atoms: vec![
                carbon(c1), carbon(c2), hydrogen(h1), hydrogen(h2),
                hydrogen(h3), hydrogen(h4), hydrogen(h5), hydrogen(h6),
                oxygen(o)
                ],
            links: vec![
                link(c1, c2), link(c1, o), link(h1, c1), link(h2, c1),
                link(h3, o), link(h4, c2), link(h5, c2), link(h6, c2),
                ],
        }
    }
}

impl MoleculeObject {
    pub fn build(self) -> Composite {
        Composite::Molecule(self)
    }
}


impl Hit for MoleculeObject {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t.max;
        for obj in &self.atoms {
            match obj.hit(r, Interval { max: closest, ..t }) {
                None => (),
                Some(rec) => {
                    closest = rec.t;
                    record = Some(rec);
                }
            }
        }
        for obj in &self.links {
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
