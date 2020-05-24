use pyo3::prelude::*;

use libtrace::internal;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Vec3 {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64,
}

#[pymethods]
impl Vec3 {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Vec3 {
    pub fn to_internal(self) -> internal::Vec3 {
        internal::Vec3(self.x, self.y, self.z)
    }
}
