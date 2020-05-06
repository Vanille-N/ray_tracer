use crate::hitable::*;
use crate::primitives::*;
use crate::rgb::RGB;
use crate::vec3::Vec3;

pub struct Erlenmeyer {
    pub a: Vec3,
    pub size: f64,
}

impl Erlenmeyer {
    pub fn build(self) -> Composite {
        let up = Vec3(0.0, self.size, 0.0);
        let side = Vec3(self.size, 0.0, 0.0);
        let glass = Texture::Dielectric(RGB(0.8, 0.8, 0.8), 1.3);
        let anti_glass = Texture::Dielectric(RGB(0.8, 0.8, 0.8), 1./1.3);
        let water = Texture::Dielectric(RGB(0.5, 0.8, 1.0), 1.4);
        let len = self.size;
        let e = 0.05;
        let theta = 0.4;
        let hgt = 2.0;
        let epsilon = 0.1 * hgt;
        let tot_hgt = hgt * 1.3;
        let liq_hgt = hgt * 0.2;
        let neck_start = 0.75 * hgt;
        let base = Cone {
            orig: up * hgt,
            dir: -up,
            angle: theta,
            begin: len * (hgt - neck_start),
            end: len * hgt,
            texture: glass,
        }
        .build()
        .remove(Cone {
            orig: up * (hgt - e/theta.sin()),
            dir: -up,
            angle: theta,
            begin: len * 0.0,
            end: len * (hgt - e/theta.sin() - e),
            texture: anti_glass,
        }
        .build());

        let solute = Cone {
            orig: up * (hgt - e/theta.sin()),
            dir: -up,
            angle: theta,
            begin: len * (hgt - e/theta.sin() - e - liq_hgt),
            end: len * (hgt - e/theta.sin() - e),
            texture: water,
        }
        .build()
        .wrap();

        let neck = Cylinder {
            center1: up * neck_start,
            center2: up * tot_hgt,
            radius: (hgt - neck_start) * theta.tan(),
            texture: glass,
        }
        .build()
        .remove(Cylinder {
            center1: up * (neck_start - epsilon),
            center2: up * (tot_hgt + epsilon),
            radius: (hgt - neck_start - e/theta.sin()) * theta.tan(),
            texture: anti_glass,
        }
        .build());

        let shaker = EmptyCylinder {
            center1: up * (tot_hgt + 4.*e) - side * e * 3.5,
            center2: up * e * 1.5 + side * e * 10.,
            radius: e / 2.,
            texture: Texture::Lambertian(RGB(0.5, 0.5, 0.5)),
        }
        .build()
        .wrap();

        vec![base, solute, neck, shaker]
    }
}
