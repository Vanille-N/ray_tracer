use pyo3::prelude::*;
use pyo3::{PyNumberProtocol, PyObjectProtocol};

use pytrace_core::internal;

#[pyclass]
#[derive(Clone, Copy)]
#[text_signature = "(x, y, z)"]
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

#[pyproto]
impl PyObjectProtocol for Vec3 {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!("⟨{}, {}, {}⟩", self.x, self.y, self.z))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!("Vector⟨{}, {}, {}⟩", self.x, self.y, self.z))
    }
}

impl Vec3 {
    pub fn to_internal(self) -> internal::Vec3 {
        internal::Vec3(self.x, self.y, self.z)
    }
}

#[pyproto]
impl PyNumberProtocol for Vec3 {
    fn __add__(lhs: Vec3, rhs: Vec3) -> PyResult<Vec3> {
        Ok(Vec3{x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z})
    }


    fn __neg__(self) -> PyResult<Vec3> {
        Ok(Vec3{x: -self.x, y: -self.y, z: -self.z})
    }

    fn __sub__(lhs: Vec3, rhs: Vec3) -> PyResult<Vec3> {
        Ok(Vec3{x: lhs.x - rhs.x, y: lhs.y - rhs.y, z: lhs.z - rhs.z})
    }

    fn __mul__(lhs: Vec3, rhs: f64) -> PyResult<Vec3> {
        Ok(Vec3{x: lhs.x * rhs, y: lhs.y * rhs, z: lhs.z * rhs})
    }

    fn __truediv__(lhs: Vec3, rhs: f64) -> PyResult<Vec3> {
        Ok(Vec3{x: lhs.x / rhs, y: lhs.y / rhs, z: lhs.z / rhs})
    }
}
