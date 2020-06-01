use crate::internal::*;

/// Abstraction for the point of view chosen to take the scene from
#[derive(Clone)]
pub struct Camera {
    /// Vertex of the cone of the field of view
    orig: Vec3,
    /// Bottom left corner of the view
    low_left: Vec3,
    /// Direction of the bottom edge of the view
    horiz: Vec3,
    /// Direction of the left edge of the view
    vert: Vec3,
}

impl Camera {
    /// Build a camera from its position relative to the origin of the space
    ///
    /// eye: vertex of the field of view
    ///
    /// vert: vertical direction
    ///
    /// vfov: field of view (degrees)
    ///
    /// ratio: width/height aspect ratio
    pub fn new_absolute(
        eye: Vec3,
        target: Vec3,
        vert: Vec3,
        vfov: f64,
        ratio: f64,
    ) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let half_hgt = (theta / 2.).tan();
        let half_wth = ratio * half_hgt;
        let w = (eye - target).unit();
        let u = vert.cross(w);
        let v = w.cross(u);
        Camera {
            orig: eye,
            low_left: eye - u * half_wth - v * half_hgt - w,
            horiz: u * 2. * half_wth,
            vert: v * 2. * half_hgt,
        }
    }

    /// Build a camera from its position relative to the point aimed at
    ///
    /// angle: rotation around the target (degrees)
    ///
    /// rise: angle above target (degrees, use negative value for low-angle shot)
    /// dist: distance to target
    /// tilt: 0 for vertical field of view
    /// aperture: vertical field of view (degrees)
    /// ratio: width/height aspect ratio
    pub fn new_relative(
        target: Vec3,
        angle: f64,
        rise: f64,
        dist: f64,
        tilt: f64,
        aperture: f64,
        ratio: f64,
    ) -> Self {
        let theta = aperture * std::f64::consts::PI / 180.;
        let half_hgt = (theta / 2.).tan();
        let half_wth = ratio * half_hgt;
        let eye = {
            let angle_rad = angle * std::f64::consts::PI / 180.;
            let rise_rad = rise * std::f64::consts::PI / 180.;
            let x = angle_rad.sin();
            let z = angle_rad.cos();
            let y = rise_rad.sin();
            Vec3(x, y, z).unit() * dist + target
        };
        let w = (eye - target).unit();
        let vert = {
            let tilt_rad = tilt * std::f64::consts::PI / 180.;
            let up = Vec3(0.0, 1.0, 0.0);
            let horiz = up.cross(w);
            up * tilt_rad.cos() + horiz * tilt_rad.sin()
        };
        let u = vert.cross(w);
        let v = w.cross(u);
        Camera {
            orig: eye,
            low_left: eye - u * half_wth - v * half_hgt - w,
            horiz: u * 2. * half_wth,
            vert: v * 2. * half_hgt,
        }
    }

    /// Calculate the direction of a ray given by the position on the image of its destination
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.low_left + self.horiz * u + self.vert * v - self.orig,
        }
    }
}
