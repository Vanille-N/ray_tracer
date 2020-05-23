use crate::external::*;
use crate::internal;
use pyo3::prelude::*;
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

    pub fn wrap(self) -> InterTree {
        InterTree::Item(self)
    }
}

pub enum Interaction {
    Inter,
    Diff,
    Union,
}

pub enum InterTree {
    Item(Primitive),
    Node(Interaction, Box<InterTree>, Box<InterTree>),
}

impl InterTree {
    pub fn inter(self, other: Self) -> Self {
        Self::Node(Interaction::Inter, Box::new(self), Box::new(other))
    }

    pub fn diff(self, other: Self) -> Self {
        Self::Node(Interaction::Diff, Box::new(self), Box::new(other))
    }

    pub fn union(self, other: Self) -> Self {
        Self::Node(Interaction::Union, Box::new(self), Box::new(other))
    }

    pub fn canonical(self) -> Vec<internal::Interaction> {
        unimplemented!()
    }
}
