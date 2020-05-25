use crate::internal;
use pyo3::prelude::*;
use std::sync::Arc;

pub trait ToInternal {
    fn to_internal(&self) -> internal::Primitive;
}

#[pyclass]
#[derive(Clone)]
pub struct Construct {
    pub contents: InterTree,
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

    pub fn wrap(self) -> Construct {
        InterTree::Item(self).wrap()
    }
}

#[derive(Copy, Clone)]
pub enum Interaction {
    Inter,
    Diff,
    Union,
}

#[derive(Clone)]
pub enum InterTree {
    Item(Primitive),
    Node(Interaction, Box<InterTree>, Box<InterTree>),
}

impl InterTree {
    pub fn wrap(self) -> Construct {
        Construct {
            contents: self,
        }
    }

    pub fn inter(self, other: Self) -> Self {
        Self::Node(Interaction::Inter, Box::new(self), Box::new(other))
    }

    pub fn diff(self, other: Self) -> Self {
        Self::Node(Interaction::Diff, Box::new(self), Box::new(other))
    }

    pub fn union(self, other: Self) -> Self {
        Self::Node(Interaction::Union, Box::new(self), Box::new(other))
    }

    pub fn canonical(&self) -> Vec<internal::Interaction> {
        match self {
            Self::Item(p) => vec![p.clone().extract()],
            Self::Node(inter, a, b) => {
                let a_can = a.canonical();
                let b_can = b.canonical();
                match inter {
                    Interaction::Union => vec_union(&a_can, &b_can),
                    Interaction::Inter => {
                        // (A \ B) & (C \ D) = (A & C) \ (B | D)
                        let mut res = Vec::new();
                        for x in &a_can {
                            for y in &b_can {
                                let internal::Interaction(x_in, x_out) = x;
                                let internal::Interaction(y_in, y_out) = y;
                                res.push(internal::Interaction(
                                    vec_union(x_in, y_in),
                                    vec_union(x_out, y_out),
                                ));
                            }
                        }
                        res
                    }
                    Interaction::Diff => {
                        // (A \ B) \ (C \ D) = (A \ (B | C)) | ((A & D) \ B)
                        let mut res = a_can;
                        for y in b_can {
                            let acc = res;
                            res = Vec::new();
                            for x in acc {
                                let internal::Interaction(x_in, x_out) = &x;
                                let internal::Interaction(y_in, y_out) = &y;
                                res.push(internal::Interaction(x_in.to_vec(), vec_union(&x_out, &y_in)));
                                res.push(internal::Interaction(vec_union(&x_in, &y_out), x_out.to_vec()));
                            }
                        }
                        res
                    }
                }
            }
        }
    }
}


fn vec_union<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut res = Vec::new();
    for x in a {
        res.push(x.clone());
    }
    for x in b {
        res.push(x.clone());
    }
    res
}

#[pymethods]
impl Construct {
    pub fn inter(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().inter(other.contents.clone()),
        }
    }

    pub fn union(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().union(other.contents.clone()),
        }
    }

    pub fn diff(&self, other: &Construct) -> Self {
        Self {
            contents: self.contents.clone().diff(other.contents.clone()),
        }
    }
}
