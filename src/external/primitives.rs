use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::internal;
use crate::external::*;

// I hope to eventually be able to shorten this file with macros...

#[pyclass]
#[derive(Copy, Clone)]
pub struct Sphere {
    #[pyo3(get, set)]
    pub center: Vec3,
    #[pyo3(get, set)]
    pub radius: f64,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Sphere {
    #[new]
    pub fn new(center: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center, radius, texture }
    }
}

impl ToInternal for Sphere {
    fn to_internal(self) -> internal::Primitive {
        internal::Sphere {
            center: self.center.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct InfinitePlane {
    #[pyo3(get, set)]
    pub orig: Vec3,
    #[pyo3(get, set)]
    pub normal: Vec3,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl InfinitePlane {
    #[new]
    pub fn new(orig: Vec3, normal: Vec3, texture: Texture) -> Self {
        Self { orig, normal, texture }
    }
}

impl ToInternal for InfinitePlane {
    fn to_internal(self) -> internal::Primitive {
        internal::InfinitePlane {
            orig: self.orig.to_internal(),
            normal: self.normal.to_internal(),
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Triangle {
    #[pyo3(get, set)]
    pub a: Vec3,
    #[pyo3(get, set)]
    pub u: Vec3,
    #[pyo3(get, set)]
    pub v: Vec3,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Triangle {
    #[new]
    pub fn new(a: Vec3, u: Vec3, v: Vec3, texture: Texture) -> Self {
        Self { a, u, v, texture }
    }
}

impl ToInternal for Triangle {
    fn to_internal(self) -> internal::Primitive {
        internal::Triangle {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Parallelogram {
    #[pyo3(get, set)]
    pub a: Vec3,
    #[pyo3(get, set)]
    pub u: Vec3,
    #[pyo3(get, set)]
    pub v: Vec3,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Parallelogram {
    #[new]
    pub fn new(a: Vec3, u: Vec3, v: Vec3, texture: Texture) -> Self {
        Self { a, u, v, texture }
    }
}

impl ToInternal for Parallelogram {
    fn to_internal(self) -> internal::Primitive {
        internal::Parallelogram {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            texture: self.texture.to_internal()
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Rhomboid {
    #[pyo3(get, set)]
    pub a: Vec3,
    #[pyo3(get, set)]
    pub u: Vec3,
    #[pyo3(get, set)]
    pub v: Vec3,
    #[pyo3(get, set)]
    pub w: Vec3,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Rhomboid {
    #[new]
    pub fn new(a: Vec3, u: Vec3, v: Vec3, w: Vec3, texture: Texture) -> Self {
        Self { a, u, v, w, texture }
    }
}

impl ToInternal for Rhomboid {
    fn to_internal(self) -> internal::Primitive {
        internal::Rhomboid {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            w: self.w.to_internal(),
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct EmptyCylinder {
    #[pyo3(get, set)]
    pub center1: Vec3,
    #[pyo3(get, set)]
    pub center2: Vec3,
    #[pyo3(get, set)]
    pub radius: f64,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl EmptyCylinder {
    #[new]
    pub fn new(center1: Vec3, center2: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center1, center2, radius, texture }
    }
}

impl ToInternal for EmptyCylinder {
    fn to_internal(self) -> internal::Primitive {
        internal::EmptyCylinder {
            center1: self.center1.to_internal(),
            center2: self.center2.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Disc {
    #[pyo3(get, set)]
    pub center: Vec3,
    #[pyo3(get, set)]
    pub normal: Vec3,
    #[pyo3(get, set)]
    pub radius: f64,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Disc {
    #[new]
    pub fn new(center: Vec3, normal: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center, normal, radius, texture }
    }
}

impl ToInternal for Disc {
    fn to_internal(self) -> internal::Primitive {
        internal::Disc {
            center: self.center.to_internal(),
            normal: self.normal.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }.build()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Cylinder {
    #[pyo3(get, set)]
    pub center1: Vec3,
    #[pyo3(get, set)]
    pub center2: Vec3,
    #[pyo3(get, set)]
    pub radius: f64,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl Cylinder {
    #[new]
    pub fn new(center1: Vec3, center2: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center1, center2, radius, texture }
    }
}

impl ToInternal for Cylinder {
    fn to_internal(self) -> internal::Primitive {
        internal::Cylinder {
            center1: self.center1.to_internal(),
            center2: self.center2.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }.build()
    }
}


#[pyclass]
#[derive(Clone, Copy)]
pub struct EmptyCone {
    #[pyo3(get, set)]
    pub orig: Vec3,
    #[pyo3(get, set)]
    pub dir: Vec3,
    #[pyo3(get, set)]
    pub angle: f64,
    #[pyo3(get, set)]
    pub begin: f64,
    #[pyo3(get, set)]
    pub end: f64,
    #[pyo3(get, set)]
    pub texture: Texture,
}

#[pymethods]
impl EmptyCone {
    #[new]
    pub fn new(orig: Vec3, dir: Vec3, angle: f64, begin: f64, end: f64, texture: Texture) -> Self {
        Self { orig, dir, angle, begin, end, texture }
    }
}

impl ToInternal for EmptyCone {
    fn to_internal(self) -> Primitive {
        internal::EmptyCone {
            orig: self.orig.to_internal(),
            dir: self.dir.to_internal(),
            angle: self.angle,
            begin: self.begin,
            end: self.end,
            texture: self.texture.to_internal(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Cone {
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
}
