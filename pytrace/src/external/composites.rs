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

macro_rules! internalize {
    ( $caller:ident, $member:ident, f64 ) => {
        $caller.$member
    };
    ( $caller:ident, $member:ident, $t:tt ) => {
        $caller.$member.to_internal()
    };
}

macro_rules! dvp {
    ($( $name:ident { $( $member:ident : $t:tt, )* } )*) => {
        $(
        #[pyclass]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                #[pyo3(get, set)] pub $member: $t,
            )*
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new( $( $member: $t ),* ) -> Self {
                Self {
                    $( $member, )*
                }
            }

            #[text_signature = "($self)"]
            pub fn build(self) -> Prebuilt {
                Prebuilt {
                    contents: Arc::new(self)
                }
            }
        }

        impl Develop for $name {
            fn develop(&self) -> internal::Composite {
                composite:: $name {
                    $( $member: internalize![self, $member, $t], )*
                }.build()
            }
        }
        )*
    }
}

dvp! {
    Axes {
        scale: f64,
    }
}
