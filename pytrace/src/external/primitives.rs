use crate::external::*;
use pyo3::prelude::*;
use pytrace_core::internal;
use std::sync::Arc;

macro_rules! internalize {
    ( $caller:ident, $member:ident, f64 ) => {
        $caller.$member
    };
    ( $caller:ident, $member:ident, $t:tt ) => {
        $caller.$member.to_internal()
    };
}

macro_rules! dvp {
    ($name:ident, $( $member:ident : $t:tt, )*) => {
        #[pyclass]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                #[pyo3(get, set)] pub $member: $t,
            )*
            #[pyo3(get, set)] pub texture: Texture,
        }

        #[pymethods]
        impl $name {
            #[new]
            #[allow(clippy::new_ret_no_self)]
            pub fn new( $( $member: $t, )* texture: Texture ) -> Construct {
                Primitive {
                    obj: Arc::new(Self {
                        $( $member, )*
                        texture,
                    }),
                }.wrap()
            }
        }

        impl ToInternal for $name {
            fn to_internal(&self) -> internal::Primitive {
                internal:: $name {
                    $( $member: internalize![self, $member, $t], )*
                    texture: self.texture.to_internal(),
                }.build()
            }
        }
    }
}

dvp! {Sphere,
    center: Vec,
    radius: f64,
}

dvp! {InfinitePlane,
    orig: Vec,
    normal: Vec,
}

dvp! {Triangle,
    a: Vec,
    u: Vec,
    v: Vec,
}

dvp! {Parallelogram,
    a: Vec,
    u: Vec,
    v: Vec,
}

dvp! {Rhomboid,
    a: Vec,
    u: Vec,
    v: Vec,
    w: Vec,
}

dvp! {EmptyCylinder,
    center1: Vec,
    center2: Vec,
    radius: f64,
}

dvp! {Disc,
    center: Vec,
    normal: Vec,
    radius: f64,
}

dvp! {Cylinder,
    center1: Vec,
    center2: Vec,
    radius: f64,
}

dvp! {EmptyCone,
    orig: Vec,
    dir: Vec,
    angle: f64,
    begin: f64,
    end: f64,
}

dvp! {Cone,
    orig: Vec,
    dir: Vec,
    angle: f64,
    begin: f64,
    end: f64,
}
