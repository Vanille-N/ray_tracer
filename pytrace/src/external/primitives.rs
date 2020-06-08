use crate::external::*;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
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

macro_rules! printable {
    ( $caller:ident, $item:ident, f64 ) => {
        $caller.$item
    };
    ( $caller:ident, $item:ident, $t:tt ) => {
        $caller.$item.__repr__().ok().unwrap()
    };
}

macro_rules! dvp {
    (
        $(
            #[sig=$sig:tt]
            #[repr=$repr:tt]
            #[str=$str:tt]
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

                fn display(&self) -> String {
                    self.__str__().ok().unwrap()
                }
            }

            #[pyproto]
            impl PyObjectProtocol for $name {
                fn __repr__(self) -> PyResult<String> {
                    Ok(format!($repr, $( printable!(self, $member, $t) ),*))
                }

                fn __str__(self) -> PyResult<String> {
                    Ok(format!($str, $( printable!(self, $member, $t) ),*))
                }
            }
        )*
    }
}

dvp! {
    #[sig="(center, radius, texture, /)"]
    #[repr="Sphere({}, {})"]
    #[str="<Sphere object at {} with radius {}>"]
    Sphere {
        center: Vec (center),
        radius: f64 (radius),
    };

    #[sig="(origin, normal, texture, /)"]
    #[repr="InfinitePlane({}, {})"]
    #[str="<InfinitePlane object at {} with normal {}>"]
    InfinitePlane {
        origin: Vec (orig),
        normal: Vec (normal),
    };

    #[sig="(vertex, edge1, edge2, texture, /)"]
    #[repr="Triangle({}, {}, {})"]
    #[str="<Triangle object at {} with edges {}, {}>"]
    Triangle {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
    };

    #[sig="(vertex, edge1, edge2, texture, /)"]
    #[repr="Parallelogram({}, {}, {})"]
    #[str="<Parallelogram object at {} with edges {}, {}>"]
    Parallelogram {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
    };

    #[sig="(vertex, edge1, edge2, edge3, texture, /)"]
    #[repr="Rhomboid({}, {}, {}, {})"]
    #[str="<Rhomboid object at {} with edges {}, {}, {}>"]
    Rhomboid {
        vertex: Vec (a),
        edge1: Vec (u),
        edge2: Vec (v),
        edge3: Vec (w),
    };

    #[sig="(center1, center2, radius, texture, /)"]
    #[repr="EmptyCylinder({}, {}, {})"]
    #[str="<EmptyCylinder object between {} and {} with radius {}>"]
    EmptyCylinder {
        center1: Vec (center1),
        center2: Vec (center2),
        radius: f64 (radius),
    };

    #[sig="(center, normal, radius, texture, /)"]
    #[repr="Disc({}, {}, {})"]
    #[str="<Disc object at {} with normal {} and radius {}>"]
    Disc {
        center: Vec (center),
        normal: Vec (normal),
        radius: f64 (radius),
    };

    #[sig="(center1, center2, radius, texture, /)"]
    #[repr="Cylinder({}, {}, {})"]
    #[str="<Cylinder object between {} and {} with radius {}>"]
    Cylinder {
        center1: Vec (center1),
        center2: Vec (center2),
        radius: f64 (radius),
    };

    #[sig="(vertex, direction, angle, begin, end, texture, /)"]
    #[repr="EmptyCone({}, {}, {}, {}, {})"]
    #[str="<EmptyCone object at {} with direction {}, angle {}, from {} to {}>"]
    EmptyCone {
        vertex: Vec (orig),
        direction: Vec (dir),
        angle: f64 (angle),
        begin: f64 (begin),
        end: f64 (end),
    };

    #[sig="(vertex, direction, angle, begin, end, texture, /)"]
    #[repr="Cone({}, {}, {}, {}, {})"]
    #[str="<Cone object at {} with direction {}, angle {}, from {} to {}>"]
    Cone{
        vertex: Vec (orig),
        direction: Vec (dir),
        angle: f64 (angle),
        begin: f64 (begin),
        end: f64 (end),
    };
}
