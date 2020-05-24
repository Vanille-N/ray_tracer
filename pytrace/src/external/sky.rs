use crate::external::*;
use libtrace::internal;

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

    #[text_signature = "()"]
    #[staticmethod]
    pub fn uniform(c: RGB) -> Self {
        Self {
            contents: internal::Sky::uniform(c.to_internal()),
        }
    }
}

impl Sky {
    pub fn to_internal(&self) -> internal::Sky {
        self.contents.clone()
    }
}
