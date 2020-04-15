use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Camera {
    orig: Vec3,
    low_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_absolute(eye: Vec3, target: Vec3, vertical: Vec3, vfov: f64, ratio: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let half_hgt = (theta / 2.).tan();
        let half_wth = ratio * half_hgt;
        let w = (eye - target).unit();
        let u = vertical.cross(&w);
        let v = w.cross(&u);
        Camera {
            orig: eye,
            low_left: eye - u * half_wth - v * half_hgt - w,
            horizontal: u * 2. * half_wth,
            vertical: v * 2. * half_hgt,
        }
    }

    pub fn new_relative(target: Vec3, angle: f64, rise: f64, distance: f64, tilt: f64, aperture: f64, ratio: f64) -> Self {
        let theta = aperture * std::f64::consts::PI / 180.;
        let half_hgt = (theta / 2.).tan();
        let half_wth = ratio * half_hgt;
        let eye = {
            let angle_rad = angle * std::f64::consts::PI / 180.;
            let rise_rad = rise * std::f64::consts::PI / 180.;
            let x = angle_rad.cos();
            let z = angle_rad.sin();
            let y = rise_rad.sin();
            Vec3::new(x, y, z).unit() * distance
        };
        let w = (eye - target).unit();
        let vertical = {
            let tilt_rad = tilt * std::f64::consts::PI / 180.;
            let up = Vec3::new(0.0, 1.0, 0.0);
            let horiz = up.cross(&w);
            up * tilt_rad.cos() + horiz * tilt_rad.sin()
        };
        let u = vertical.cross(&w);
        let v = w.cross(&u);
        Camera {
            orig: eye,
            low_left: eye - u * half_wth - v * half_hgt - w,
            horizontal: u * 2. * half_wth,
            vertical: v * 2. * half_hgt,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.low_left + self.horizontal * u + self.vertical * v - self.orig,
        }
    }
}
