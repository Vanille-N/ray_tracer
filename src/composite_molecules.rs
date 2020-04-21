use crate::hitable::*;
use crate::vec3::Vec3;
use crate::rgb::RGB;
use crate::primitives::*;

const BLACK: RGB = RGB { r: 0.05, g: 0.05, b: 0.05 };
const RED: RGB = RGB { r: 0.9, g: 0.1, b: 0.1 };
const BLUE: RGB = RGB { r: 0.0, g: 0.2, b: 0.7 };
const WHITE: RGB = RGB { r: 0.9, g: 0.9, b: 0.9 };
const LGREY: RGB = RGB { r: 0.7, g: 0.7, b: 0.7 };

const CARBON: Texture = Texture::Metal(BLACK, 0.0);
const OXYGEN: Texture = Texture::Metal(RED, 0.0);
const HYDROGEN: Texture = Texture::Metal(WHITE, 0.0);
const NITROGEN: Texture = Texture::Metal(BLUE, 0.0);

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

fn atom_builder(r: f64, texture: Texture) -> Box<dyn Fn(Vec3) -> Sphere> {
    Box::new(move |u| Sphere {
        center: u,
        radius: r,
        texture,
    })
}

fn link_builder(r: f64) -> Box<dyn Fn(Vec3, Vec3) -> EmptyCylinder> {
    Box::new(move |c1, c2| EmptyCylinder {
        center1: c1,
        center2: c2,
        radius: r,
        texture: Texture::Lambertian(LGREY),
    })
}

fn double_builder(r: f64) -> Box<dyn Fn(Vec3, Vec3, Vec3) -> [EmptyCylinder; 2]> {
    Box::new(move |c1, c2, c3| {
        let orth = (c2 - c1).cross(&c3).unit() * (c2 - c1).len();
        [
            EmptyCylinder {
                center1: c1 + orth * r * 3.0,
                center2: c2 + orth * r * 3.0,
                radius: r,
                texture: Texture::Lambertian(LGREY),
            },
            EmptyCylinder {
                center1: c1 - orth * r * 3.0,
                center2: c2 - orth * r * 3.0,
                radius: r,
                texture: Texture::Lambertian(LGREY),
            },
        ]
    })
}

pub fn dimensions(len: f64) -> [f64; 5] {
    [
        len * 1.4, // big atoms (C, N, O) radius
        len * 0.8, // small atoms (H) radius
        len * 0.3, // link radius
        len * 5.0, // long link length
        len * 3.0, // short link length
    ]
}

fn triple_builder(r: f64) -> Box<dyn Fn(Vec3, Vec3, Vec3) -> [EmptyCylinder; 3]> {
    Box::new(move |c1, c2, c3| {
        let axis = c2 - c1;
        let len = axis.len();
        let orth1 = axis.cross(&c3).unit() * len;
        let orth2 = axis.cross(&orth1).unit() * len;
        [
            EmptyCylinder {
                center1: c1 + orth1 * r * 4.5,
                center2: c2 + orth1 * r * 4.5,
                radius: r,
                texture: Texture::Lambertian(LGREY),
            },
            EmptyCylinder {
                center1: c1 + (orth2 * 0.5 - orth1 * 0.78) * r * 4.5,
                center2: c2 + (orth2 * 0.5 - orth1 * 0.78) * r * 4.5,
                radius: r,
                texture: Texture::Lambertian(LGREY),
            },
            EmptyCylinder {
                center1: c1 + (-orth2 * 0.5 - orth1 * 0.78) * r * 4.5,
                center2: c2 + (-orth2 * 0.5 - orth1 * 0.78) * r * 4.5,
                radius: r,
                texture: Texture::Lambertian(LGREY),
            },
        ]
    })
}


#[allow(unused_variables)]
impl Molecule {
    fn directions(self) -> [Vec3; 7] {
        let zz = self.up.unit();
        let yy = self.fwd.cross(&self.up).unit();
        let xx = zz.cross(&yy).unit();

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
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let carbon = atom_builder(rad1, CARBON);
        let oxygen = atom_builder(rad1, OXYGEN);
        let nitrogen = atom_builder(rad1, NITROGEN);
        let hydrogen = atom_builder(rad2, HYDROGEN);
        let link = link_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

        let c1 = self.c_ref;
        let c2 = c1 - t * len1;
        let c3 = c1 - u * len1;
        let c4 = c2 + v * len1;
        let c5 = c3 + v * len1;
        let c6 = c4 - u * len1;
        let n = c6 + v * len1;
        let c7 = n + x * len1;
        let o = c1 - v * len1;

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
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let oxygen = atom_builder(rad1, OXYGEN);
        let hydrogen = atom_builder(rad2, HYDROGEN);
        let link = link_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

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
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let carbon = atom_builder(rad1, CARBON);
        let hydrogen = atom_builder(rad2, HYDROGEN);
        let link = link_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

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
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let carbon = atom_builder(rad1, CARBON);
        let oxygen = atom_builder(rad1, OXYGEN);
        let hydrogen = atom_builder(rad2, HYDROGEN);
        let link = link_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

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

    pub fn carbon_dioxide(self) -> MoleculeObject {
        let len = self.fwd.len();
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let oxygen = atom_builder(rad1, OXYGEN);
        let carbon = atom_builder(rad1, CARBON);
        let link = double_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

        let c = self.c_ref;
        let o1 = c + x * len1;
        let o2 = c - x * len1;

        let [l1, l2] = link(c, o1, v);
        let [l3, l4] = link(c, o2, v);

        MoleculeObject {
            atoms: vec![
                oxygen(o1), oxygen(o2), carbon(c),
                ],
            links: vec![
                l1, l2, l3, l4,
                ],
        }
    }

    pub fn dinitrogen(self) -> MoleculeObject {
        let len = self.fwd.len();
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let nitrogen = atom_builder(rad1, NITROGEN);
        let link = triple_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

        let n1 = self.c_ref;
        let n2 = n1 + x * len1;

        let [l1, l2, l3] = link(n1, n2, v);

        MoleculeObject {
            atoms: vec![
                nitrogen(n1), nitrogen(n2),
                ],
            links: vec![
                l1, l2, l3,
                ],
        }
    }

    pub fn benzene(self) -> MoleculeObject {
        let len = self.fwd.len();
        let [rad1, rad2, rad3, len1, len2] = dimensions(len);
        let carbon = atom_builder(rad1, CARBON);
        let hydrogen = atom_builder(rad2, HYDROGEN);
        let link = link_builder(rad3);
        let double = double_builder(rad3);

        let [t, u, v, w, x, y, z] = self.directions();

        let c1 = self.c_ref;
        let c2 = c1 + v * len1;
        let c3 = c2 + x * len1;
        let c4 = c3 - y * len1;
        let c5 = c4 - v * len1;
        let c6 = c5 - x * len1;

        let h1 = c1 - x * len2;
        let h2 = c2 + y * len2;
        let h3 = c3 + v * len2;
        let h4 = c4 + x * len2;
        let h5 = c5 - y * len2;
        let h6 = c6 - v * len2;

        let (l12, [l23a, l23b], l34, [l45a, l45b], l56, [l61a, l61b]) = (link(c1, c2), double(c2, c3, u), link(c3, c4), double(c4, c5, u), link(c5, c6), double(c6, c1, u));

        MoleculeObject {
            atoms: vec![carbon(c1), carbon(c2), carbon(c3), carbon(c4), carbon(c5), carbon(c6), hydrogen(h1), hydrogen(h2), hydrogen(h3), hydrogen(h4), hydrogen(h5), hydrogen(h6)],
            links: vec![l12, l23a, l23b, l34, l45a, l45b, l56, l61a, l61b, link(c1, h1), link(c2, h2), link(c3, h3), link(c4, h4), link(c5, h5), link(c6, h6)],
        }
    }

    pub fn test(self) -> MoleculeObject {
        let len = self.fwd.len();
        let [rad1, _, rad3, len1, _] = dimensions(len);

        let [t, u, v, w, x, y, z] = self.directions();
        let link = link_builder(rad3);

        let o = self.c_ref;
        let ot = o + t * len1;
        let ou = o + u * len1;
        let ov = o + v * len1;
        let ow = o + w * len1;
        let ox = o + x * len1;
        let oy = o + y * len1;
        let oz = o + z * len1;

        MoleculeObject {
            atoms: vec![
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(0., 0., 0.)))(o),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(1., 0., 0.)))(ot),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(0., 1., 0.)))(ou),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(0., 0., 1.)))(ov),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(1., 1., 0.)))(ow),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(1., 0., 1.)))(ox),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(0., 1., 1.)))(oy),
                atom_builder(rad1,
                    Texture::Lambertian(RGB::new(1., 1., 1.)))(oz),
                ],
            links: vec![link(o, ot), link(o, ou), link(o, ov), link(o, ow), link(o, ox), link(o, oy), link(o, oz)],
        }
    }

}

impl MoleculeObject {
    pub fn build(self) -> Composite {
        let mut res = Vec::new();
        for x in self.atoms {
            res.push(x.build().wrap());
        }
        for x in self.links {
            res.push(x.build().wrap());
        }
        res
    }
}
