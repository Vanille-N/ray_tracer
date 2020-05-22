use crate::external::*;
use crate::internal;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Arc;

macro_rules! internalize {
    ( $caller:ident, $member:ident, f64 ) => {
        $caller.$member
    };
    ( $caller:ident, $member:ident, $t:tt ) => {
        $caller.$member.to_internal()
    }
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
            pub fn new( $( $member: $t, )* texture: Texture ) -> Primitive {
                Primitive {
                    obj: Arc::new(Self {
                        $( $member, )*
                        texture,
                    }),
                }
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

dvp!{Sphere,
    center: Vec3,
    radius: f64,
}

dvp!{InfinitePlane,
    orig: Vec3,
    normal: Vec3,
}

dvp!{Triangle,
    a: Vec3,
    u: Vec3,
    v: Vec3,
}

dvp!{Parallelogram,
    a: Vec3,
    u: Vec3,
    v: Vec3,
}

dvp!{Rhomboid,
    a: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

dvp!{EmptyCylinder,
    center1: Vec3,
    center2: Vec3,
    radius: f64,
}

dvp!{Disc,
    center: Vec3,
    normal: Vec3,
    radius: f64,
}

dvp!{Cylinder,
    center1: Vec3,
    center2: Vec3,
    radius: f64,
}

dvp!{EmptyCone,
    orig: Vec3,
    dir: Vec3,
    angle: f64,
    begin: f64,
    end: f64,
}

dvp!{Cone,
    orig: Vec3,
    dir: Vec3,
    angle: f64,
    begin: f64,
    end: f64,
}
