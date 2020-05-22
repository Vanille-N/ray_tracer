use crate::external::*;
use crate::internal;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Arc;

pub trait ToInternal {
    fn to_internal(&self) -> internal::Primitive;
}

#[pyclass]
#[derive(Clone)]
pub struct Primitive {
    pub obj: Arc<dyn ToInternal>,
}

impl Primitive {
    pub fn extract(self) -> internal::Interaction {
        self.obj.to_internal().wrap()
    }
}

// pub enum Interaction {
//     Intersection,
//     Difference,
//     Union,
// }
//
// pub enum InterTree {
//     Item(Primitive),
//     Node(Interaction, Box<InterTree>, Box<InterTree>),
// }
