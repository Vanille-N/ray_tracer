use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::internal;
use crate::external::*;

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

impl Triangle {
    pub fn to_internal(self) -> internal::Triangle {
        internal::Triangle {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            texture: self.texture.to_internal(),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Parallelogram {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub texture: Texture,
}

#[pymethods]
impl Parallelogram {
    #[new]
    pub fn new(a: Vec3, u: Vec3, v: Vec3, texture: Texture) -> Self {
        Self { a, u, v, texture }
    }
}

impl Parallelogram {
    pub fn to_internal(self) -> internal::Parallelogram {
        internal::Parallelogram {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            texture: self.texture.to_internal()
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Rhomboid {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub texture: Texture,
}

#[pymethods]
impl Rhomboid {
    #[new]
    pub fn new(a: Vec3, u: Vec3, v: Vec3, w: Vec3, texture: Texture) -> Self {
        Self { a, u, v, w, texture }
    }
}

impl Rhomboid {
    pub fn to_internal(self) -> internal::Rhomboid {
        internal::Rhomboid {
            a: self.a.to_internal(),
            u: self.u.to_internal(),
            v: self.v.to_internal(),
            w: self.w.to_internal(),
            texture: self.texture.to_internal(),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct EmptyCylinder {
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

#[pymethods]
impl EmptyCylinder {
    #[new]
    pub fn new(center1: Vec3, center2: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center1, center2, radius, texture }
    }
}

impl EmptyCylinder {
    pub fn to_internal(self) -> internal::EmptyCylinder {
        internal::EmptyCylinder {
            center1: self.center1.to_internal(),
            center2: self.center2.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Disc {
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

#[pymethods]
impl Disc {
    #[new]
    pub fn new(center: Vec3, normal: Vec3, radius: f64, texture: Texture) -> Self {
        Self { center, normal, radius, texture }
    }
}

impl Disc {
    pub fn to_internal(self) -> internal::Disc {
        internal::Disc {
            center: self.center.to_internal(),
            normal: self.normal.to_internal(),
            radius: self.radius,
            texture: self.texture.to_internal(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cylinder {
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct EmptyCone {
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
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
