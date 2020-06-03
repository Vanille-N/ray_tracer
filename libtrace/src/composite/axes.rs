use crate::internal::*;

const RED: Texture = Texture::Lambertian(RGB(0.9, 0.1, 0.1));
const BLUE: Texture = Texture::Lambertian(RGB(0.0, 0.2, 0.7));
const GREEN: Texture = Texture::Lambertian(RGB(0.0, 0.9, 0.0));
const BLACK: Texture = Texture::Lambertian(RGB(0.01, 0.01, 0.01));

/// Axes are a debug structure that indicate the position of the origin of the space
/// as well as the x, y and z directions.
///
/// It is also a way to evaluate distances, as it includes spheres with regular spacing.
/// Its only parameter is in fact its size.
///
/// Its major downside is that being composed of almost 20 objects, it slows down rendering
/// quite a bit.
#[derive(Clone, Copy)]
pub struct Axes {
    pub scale: f64,
}

#[allow(unused_variables)]
#[allow(clippy::many_single_char_names)]
impl Axes {
    pub fn build(self) -> Composite {
        let len = self.scale;
        let rad1 = len * 0.1;
        let rad2 = len * 0.2;
        let o = Vec3(0.0, 0.0, 0.0);
        let x = Vec3(1.0, 0.0, 0.0) * len;
        let y = Vec3(0.0, 1.0, 0.0) * len;
        let z = Vec3(0.0, 0.0, 1.0) * len;
        let orig = Sphere {
            center: o,
            radius: rad2,
            texture: BLACK,
        };
        let xdir = EmptyCylinder {
            center1: o,
            center2: o + x * 5.0,
            radius: rad1,
            texture: RED,
        };
        let ydir = EmptyCylinder {
            center1: o,
            center2: o + y * 5.0,
            radius: rad1,
            texture: BLUE,
        };
        let zdir = EmptyCylinder {
            center1: o,
            center2: o + z * 5.0,
            radius: rad1,
            texture: GREEN,
        };
        let mut v = vec![orig];
        for ix in 1..=5 {
            v.push(Sphere {
                center: o + x * ix as f64,
                radius: rad2,
                texture: RED,
            });
        }
        for iy in 1..=5 {
            v.push(Sphere {
                center: o + y * iy as f64,
                radius: rad2,
                texture: BLUE,
            });
        }
        for iz in 1..=5 {
            v.push(Sphere {
                center: o + z * iz as f64,
                radius: rad2,
                texture: GREEN,
            });
        }
        let mut res = vec![
            xdir.build().wrap(),
            ydir.build().wrap(),
            zdir.build().wrap(),
        ];
        for x in v {
            res.push(x.build().wrap());
        }
        res
    }
}
