use crate::external::*;
use pyo3::prelude::*;
use pytrace_core::{composite, internal};
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct Prebuilt {
    contents: Arc<dyn Develop>,
}

impl Prebuilt {
    pub fn extract(&self) -> internal::Composite {
        self.contents.develop()
    }
}

pub trait Develop {
    fn develop(&self) -> internal::Composite;
}

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(scale, /)"]
pub struct Axes {
    #[pyo3(get, set)] pub scale: f64,
}

#[pymethods]
impl Axes {
    #[new]
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
        }
    }

    #[text_signature = "($self, /)"]
    pub fn build(self) -> Prebuilt {
        Prebuilt {
            contents: Arc::new(self)
        }
    }
}

impl Develop for Axes {
    fn develop(&self) -> internal::Composite {
        composite::Axes {
            scale: self.scale,
        }.build()
    }
}
        }

        }
    }
}

    }
}
