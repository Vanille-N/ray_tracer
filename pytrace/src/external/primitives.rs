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
    (
        $(
            #[sig=$sig:tt]
            $name:ident {
                $( $member:ident : $t:tt ($alias:ident), )*
            };
        )*
    ) => {
        $(
            #[pyclass]
            #[derive(Copy, Clone)]
            #[text_signature = $sig]
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
                        $( $alias: internalize![self, $member, $t], )*
                        texture: self.texture.to_internal(),
                    }.build()
                }
            }
        )*
    }
}

dvp! {
    #[sig="(center, radius, /)"]
    Sphere {
        center: Vec (center),
        radius: f64 (radius),
    };

    #[sig="(origin, normal, /)"]
    InfinitePlane {
        origin: Vec (orig),
        normal: Vec (normal),
    };

    #[sig="(vertex, edge1, edge2, /)"]
    Triangle {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
    };

    #[sig="(vertex, edge1, edge2, /)"]
    Parallelogram {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
    };

    #[sig="(vertex, edge1, edge2, edge3, /)"]
    Rhomboid {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
        edge3: Vec (w),
    };

    #[sig="(center1, center2, radius, /)"]
    EmptyCylinder {
        center1: Vec (center1),
        center2: Vec (center2),
        radius: f64 (radius),
    };

    #[sig="(center, normal, radius, /)"]
    Disc {
        center: Vec (center),
        normal: Vec (normal),
        radius: f64 (radius),
    };

    #[sig="(center1, center2, radius, /)"]
    Cylinder {
        center1: Vec (center1),
        center2: Vec (center2),
        radius: f64 (radius),
    };

    #[sig="(vertex, direction, angle, begin, end, /)"]
    EmptyCone {
        vertex: Vec (orig),
        direction: Vec (dir),
        angle: f64 (angle),
        begin: f64 (begin),
        end: f64 (end),
    };

    #[sig="(vertex, direction, angle, begin, end, /)"]
    Cone{
        vertex: Vec (orig),
        direction: Vec (dir),
        angle: f64 (angle),
        begin: f64 (begin),
        end: f64 (end),
    };
}
