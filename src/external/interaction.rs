use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::internal;
use crate::external::*;
use std::sync::Arc;

pub trait ToInternal {
    fn to_internal(self) -> internal::Primitive;
}

pub struct Primitive(Box<dyn ToInternal>);

pub enum Interaction {
    Intersection,
    Difference,
    Union,
}

pub enum InterTree {
    Item(Primitive),
    Node(Interaction, Box<InterTree>, Box<InterTree>),
}
