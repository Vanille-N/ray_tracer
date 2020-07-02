use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::external::Vec;
use crate::internal;

#[pyclass]
#[text_signature = "(x: float, y: float, z: float, /)"]
#[derive(Clone, Copy)]
pub struct Camera {
    pub target: internal::Vec3,
    #[pyo3(get, set)]
    pub angle: f64,
    #[pyo3(get, set)]
    pub rise: f64,
    #[pyo3(get, set)]
    pub distance: f64,
    #[pyo3(get, set)]
    pub tilt: f64,
    #[pyo3(get, set)]
    pub aperture: f64,
    #[pyo3(get, set)]
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

    #[text_signature = "($self, x: float, y: float, z: float, /)"]
    pub fn set_target(&mut self, x: f64, y: f64, z: f64) {
        self.target = internal::Vec3(x, y, z);
    }
}

#[pyproto]
impl PyObjectProtocol for Camera {
    fn __str__(self) -> PyResult<String> {
        Ok(format!(
            "Camera {{
    target:   {},
    angle:    {},
    rise:     {},
    distance: {},
    tilt:     {},
    aperture: {},
    aspect:   {},
}}",
            repr!(Vec, self.target),
            self.angle,
            self.rise,
            self.distance,
            self.tilt,
            self.aperture,
            self.aspect,
        ))
    }
}
