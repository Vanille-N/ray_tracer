use pytrace_core::internal;

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Sky {
    contents: internal::Sky,
}

#[pymethods]
impl Sky {
    #[new]
    pub fn new(file: &str) -> Self {
        Self {
            contents: internal::Sky::new(file),
        }
    }

    #[text_signature = "(r: float, g: float, b: float, /)"]
    #[staticmethod]
    pub fn uniform(r: f64, g: f64, b: f64) -> Self {
        Self {
            contents: internal::Sky::uniform(internal::RGB(r, g, b)),
        }
    }
}

impl Sky {
    pub fn to_internal(&self) -> internal::Sky {
        self.contents.clone()
    }
}
