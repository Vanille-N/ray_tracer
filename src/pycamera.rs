use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::internal;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Camera {
    pub target: internal::Vec3,
    pub angle: f64,
    pub rise: f64,
    pub distance: f64,
    pub tilt: f64,
    pub aperture: f64,
    pub aspect: f64,
}

impl Camera {
    pub fn to_internal(&self) -> internal::Camera {
        internal::Camera::new_relative(
            self.target,
            self.angle,
            self.rise,
            self.distance,
            self.tilt,
            self.aperture,
            self.aspect,
        )
    }
}

#[pymethods]
impl Camera {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            target: internal::Vec3(x, y, z),
            angle: 0.,
            rise: 0.,
            distance: 1.,
            tilt: 0.,
            aperture: 30.,
            aspect: -1.,
        }
    }

    pub fn set_target(&mut self, x: f64, y: f64, z: f64) {
        self.target = internal::Vec3(x, y, z);
    }

    pub fn set_distance(&mut self, d: f64) {
        self.distance = d;
    }

    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }

    pub fn set_rise(&mut self, r: f64) {
        self.rise = r;
    }
}
