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
    pub fn new_absolute(
        /// Vertex of the field of view
        eye: Vec3,
        target: Vec3,
        /// Vertical direction
        vert: Vec3,
        /// Field of view (degrees)
        vfov: f64,
        /// Width/height aspect ratio
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
    pub fn new_relative(
        target: Vec3,
        /// Rotation around the target (degrees)
        angle: f64,
        /// Angle above target (degrees, use negative value for low-angle shot)
        rise: f64,
        /// Distance to target
        distance: f64,
        /// 0 for vertical field of view
        tilt: f64,
        /// Vertical field of view (degrees)
        aperture: f64,
        /// Width/height aspect ratio
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
            Vec3(x, y, z).unit() * distance + target
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
