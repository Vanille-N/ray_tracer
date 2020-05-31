use pyo3::prelude::*;

use pytrace_core::internal;

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

    pub fn mul(self, f: f64) -> Self {
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }

    pub fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Vec3 {
    pub fn to_internal(self) -> internal::Vec3 {
        internal::Vec3(self.x, self.y, self.z)
    }
}

// Fails for some unknown reason...

// #[pyproto]
// impl PyNumberProtocol for Vec3 {
//     fn __add__(&self, other: &Vec3) -> PyResult<Vec3> {
//         Ok(Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z})
//     }
// }
