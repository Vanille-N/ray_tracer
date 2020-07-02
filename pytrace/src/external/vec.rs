use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyNumberProtocol, PyObjectProtocol};
use std::vec;

use pytrace_core::internal;

#[pyclass]
#[derive(Clone, Copy)]
#[text_signature = "(x: float, y: float, z: float, /)"]
pub struct Vec {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64,
}

#[pymethods]
impl Vec {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[pyproto]
impl PyObjectProtocol for Vec {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!("⟨{}, {}, {}⟩", self.x, self.y, self.z))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!("Vector⟨{}, {}, {}⟩", self.x, self.y, self.z))
    }
}

impl Vec {
    pub fn to_internal(self) -> internal::Vec3 {
        internal::Vec3(self.x, self.y, self.z)
    }

    pub fn into_iter(self) -> Iterator {
        Iterator::new(vec![self.x, self.y, self.z])
    }

    pub fn from(v: internal::Vec3) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

#[pyproto]
impl PyNumberProtocol for Vec {
    fn __add__(lhs: Vec, rhs: Vec) -> PyResult<Vec> {
        Ok(Vec {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        })
    }

    fn __neg__(self) -> PyResult<Vec> {
        Ok(Vec {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        })
    }

    fn __sub__(lhs: Vec, rhs: Vec) -> PyResult<Vec> {
        Ok(Vec {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
        })
    }

    fn __mul__(lhs: Vec, rhs: f64) -> PyResult<Vec> {
        Ok(Vec {
            x: lhs.x * rhs,
            y: lhs.y * rhs,
            z: lhs.z * rhs,
        })
    }

    fn __truediv__(lhs: Vec, rhs: f64) -> PyResult<Vec> {
        Ok(Vec {
            x: lhs.x / rhs,
            y: lhs.y / rhs,
            z: lhs.z / rhs,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Iterator {
    idx: usize,
    contents: vec::Vec<f64>,
}

impl Iterator {
    pub fn new(v: vec::Vec<f64>) -> Self {
        Self {
            idx: 0,
            contents: v,
        }
    }
}

#[pyproto]
impl PyIterProtocol for Iterator {
    fn __next__(mut item: PyRefMut<Iterator>) -> PyResult<Option<f64>> {
        if item.idx < item.contents.len() {
            let res = item.contents[item.idx];
            item.idx += 1;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

#[pyproto]
impl PyIterProtocol for Vec {
    fn __iter__(item: PyRefMut<Vec>) -> PyResult<Iterator> {
        Ok(item.into_iter())
    }
}
