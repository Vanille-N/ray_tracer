use crate::internal;
use pyo3::prelude::*;
use pyo3::{PyNumberProtocol, PyObjectProtocol};
use std::fmt;
use std::sync::Arc;
use std::vec;

pub trait ToInternal: Send + Sync {
    fn to_internal(&self) -> internal::Primitive;
    fn display(&self) -> String;
}

#[pyclass]
#[derive(Clone)]
pub struct Construct {
    pub contents: InterTree,
}

#[pyproto]
impl PyObjectProtocol for Construct {
    fn __str__(self) -> PyResult<String> {
        Ok(self.contents.display())
    }
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

    pub fn display(&self) -> String {
        self.obj.display()
    }
}

#[derive(Copy, Clone)]
pub enum Interaction {
    Inter,
    Diff,
    Union,
}

impl fmt::Display for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inter => write!(f, "&"),
            Self::Union => write!(f, "|"),
            Self::Diff => write!(f, "-"),
        }
    }
}

#[derive(Clone)]
pub enum InterTree {
    Item(Primitive),
    Node(Interaction, Box<InterTree>, Box<InterTree>),
}

impl InterTree {
    pub fn wrap(self) -> Construct {
        Construct { contents: self }
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

    // See explanations below
    pub fn canonical(&self) -> vec::Vec<internal::Interaction> {
        match self {
            Self::Item(p) => vec![p.clone().extract()],
            Self::Node(inter, a, b) => {
                let a_can = a.canonical();
                let b_can = b.canonical();
                match inter {
                    Interaction::Union => vec_union(&a_can, &b_can),
                    Interaction::Inter => {
                        // (A \ B) & (C \ D) = (A & C) \ (B | D)
                        let mut res = vec::Vec::new();
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
                            res = vec::Vec::new();
                            for x in acc {
                                let internal::Interaction(x_in, x_out) = &x;
                                let internal::Interaction(y_in, y_out) = &y;
                                for z in y_in {
                                    res.push(internal::Interaction(
                                        x_in.to_vec(),
                                        vec_union(&x_out, &[z.clone()]),
                                    ));
                                }
                                if !y_out.is_empty() {
                                    res.push(internal::Interaction(
                                        vec_union(&x_in, &y_out),
                                        x_out.to_vec(),
                                    ));
                                }
                            }
                        }
                        res
                    }
                }
            }
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Item(p) => p.display(),
            Self::Node(_, lt, rt) => {
                let s = String::from("Interaction of:");
                let s = lt.accumulate_display(s);
                rt.accumulate_display(s)
            }
        }
    }

    fn accumulate_display(&self, s: String) -> String {
        match self {
            Self::Item(p) => format!("{}\n    {}", s, p.display()),
            Self::Node(_, lt, rt) => {
                let s = lt.accumulate_display(s);
                rt.accumulate_display(s)
            }
        }
    }
}

fn vec_union<T: Clone>(a: &[T], b: &[T]) -> vec::Vec<T> {
    let mut res = vec::Vec::new();
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

#[pyproto]
impl PyNumberProtocol for Construct {
    fn __and__(lhs: Construct, rhs: Construct) -> PyResult<Construct> {
        Ok(lhs.inter(&rhs))
    }

    fn __or__(lhs: Construct, rhs: Construct) -> PyResult<Construct> {
        Ok(lhs.union(&rhs))
    }

    fn __sub__(lhs: Construct, rhs: Construct) -> PyResult<Construct> {
        Ok(lhs.diff(&rhs))
    }
}

// About the InterTree::canonical() function
//
// The internal algorithm only manages complex objects expressed as :
// Union_i ( Inter_j (A_i,j) \ Union_k (B_i,k) )
// This corresponds to a collection (i.e. union) of [Vec, Vec],
// where the first (resp. second) Vec represents all objects inside
// (resp. outside) of which we need to be.
// The user of the Python library need not be aware of this restriction,
// as the interface allows arbitrary set operations.
// This is done using a tree (an InterTree) of operations, which is then
// converted into a canonical representation before being pushed to the
// scene.
//
// The process of translating InterTree -> Vec<[Vec, Vec]> is done recursively
// by InterTree::canonical()
// It relies on the following :
// (&, |, \ represent intersection, union, difference)
// * a leaf is already under canonical representation :
//        A -> { [A, ()] }
// * an union is easy to canonicalize
//        X | Y -> { X, Y }
// * A\B & C\D = A&C \ B|D
//        [A, B] & [C, D] -> [(A.., C..), (B.., D..)]
// * & is distributive on |
//        { [A, B], [A', B'] } & { [C, D], [C', D'] }
//     -> {
//             [(A.., C..), (B.., D..)],
//             [(A'.., C..), (B'.., D..)],
//             [(A.., C'..), (B.., D'..)],
//             [(A'.., C'..), (B'.., D'..)],
//        }
// And finally:
// * { X, Y } \ { Z, W } = { X\Z\W, Y\Z\W }
// * [A, B] \ [C, ()] = [A, (B.., C..)]
// * [A, B] \ [C, D] = { [A, (B.., C..)], [(A.., D..), B] }
// These three rules are applied by iteratively removing all elements
// of b_can at each step. The result becomes the basis of the next step.
