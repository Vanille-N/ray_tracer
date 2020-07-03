use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyNumberProtocol, PyObjectProtocol};
use pytrace_core::internal;
use std::vec;

#[pyclass]
#[text_signature = "(r: float, g: float, b: float, /)"]
#[derive(Copy, Clone)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[pymethods]
impl RGB {
    #[new]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl RGB {
    pub fn to_internal(self) -> internal::RGB {
        internal::RGB(self.r, self.g, self.b)
    }

    pub fn into_iter(self) -> Iterator {
        Iterator::new(vec![self.r, self.g, self.b])
    }

    pub fn from(c: internal::RGB) -> Self {
        Self {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
}

macro_rules! color {
    ( $name:ident : $r:expr, $g:expr, $b:expr ) => {
        #[pymethods]
        impl RGB {
            #[staticmethod]
            #[text_signature = "(/)"]
            pub fn $name() -> RGB {
                RGB {
                    r: $r,
                    g: $g,
                    b: $b,
                }
            }
        }
    };
}

color!(red: 1.0, 0.0, 0.0);
color!(dkred: 0.5, 0.0, 0.0);
color!(ltred: 1.0, 0.5, 0.5);
color!(blue: 0.0, 0.0, 1.0);
color!(dkblue: 0.0, 0.0, 0.5);
color!(ltblue: 0.3, 0.6, 1.0);
color!(cyan: 0.0, 1.0, 1.0);
color!(green: 0.0, 1.0, 0.0);
color!(dkgreen: 0.0, 0.5, 0.0);
color!(ltgreen: 0.7, 1.0, 0.0);
color!(purple: 0.7, 0.0, 0.0);
color!(magenta: 1.0, 0.0, 1.0);
color!(yellow: 1.0, 1.0, 0.0);
color!(brown: 0.3, 0.2, 0.0);
color!(orange: 1.0, 0.4, 0.0);
color!(turquoise: 0.0, 0.9, 0.6);
color!(black: 0.0, 0.0, 0.0);
color!(white: 1.0, 1.0, 1.0);
color!(grey: 0.5, 0.5, 0.5);
color!(dkgrey: 0.2, 0.2, 0.2);
color!(ltgrey: 0.8, 0.8, 0.8);

#[pyclass]
#[derive(Clone, Copy)]
pub struct Texture {
    contents: internal::Texture,
}

#[pyproto]
impl PyObjectProtocol for Texture {
    fn __str__(self) -> PyResult<String> {
        match self.contents {
            internal::Texture::Lambertian(c) => {
                Ok(format!("<Lambertian Texture with color {}>", repr!(RGB, c)))
            }
            internal::Texture::Metal(c, f) => Ok(format!(
                "<Metallic Texture with color {} and fuzziness {}>",
                repr!(RGB, c),
                f
            )),
            internal::Texture::Light(c) => {
                Ok(format!("<Light Texture with color {}>", repr!(RGB, c)))
            }
            internal::Texture::Dielectric(c, n) => Ok(format!(
                "<Dielectric Texture with color {} and index {}>",
                repr!(RGB, c),
                n
            )),
        }
    }

    fn __repr__(self) -> PyResult<String> {
        match self.contents {
            internal::Texture::Lambertian(c) => Ok(format!("Lambertian[{}]", repr!(RGB, c))),
            internal::Texture::Metal(c, f) => Ok(format!("Metal[{},{}]", repr!(RGB, c), f)),
            internal::Texture::Light(c) => Ok(format!("Light[{}]", repr!(RGB, c))),
            internal::Texture::Dielectric(c, n) => {
                Ok(format!("Dielectric[{},{}]", repr!(RGB, c), n))
            }
        }
    }
}

#[pymethods]
impl Texture {
    #[staticmethod]
    #[text_signature = "(color: RGB, /)"]
    pub fn lambertian(color: RGB) -> Self {
        Self {
            contents: internal::Texture::Lambertian(color.to_internal()),
        }
    }

    #[staticmethod]
    #[text_signature = "(color: RGB, fuzzy: float, /)"]
    pub fn metal(color: RGB, fuzzy: f64) -> Self {
        Self {
            contents: internal::Texture::Metal(color.to_internal(), fuzzy),
        }
    }

    #[staticmethod]
    #[text_signature = "(color: RGB, /)"]
    pub fn light(color: RGB) -> Self {
        Self {
            contents: internal::Texture::Light(color.to_internal()),
        }
    }

    #[staticmethod]
    #[text_signature = "(color: RGB, index: float, /)"]
    pub fn dielectric(color: RGB, index: f64) -> Self {
        Self {
            contents: internal::Texture::Dielectric(color.to_internal(), index),
        }
    }
}

impl Texture {
    pub fn to_internal(self) -> internal::Texture {
        self.contents
    }
}

#[pyproto]
impl PyNumberProtocol for RGB {
    fn __add__(lhs: RGB, rhs: RGB) -> PyResult<RGB> {
        Ok(RGB {
            r: lhs.r + rhs.r,
            g: lhs.g + rhs.g,
            b: lhs.b + rhs.b,
        })
    }

    fn __sub__(lhs: RGB, rhs: RGB) -> PyResult<RGB> {
        Ok(RGB {
            r: lhs.r - rhs.r,
            g: lhs.g - rhs.g,
            b: lhs.b - rhs.b,
        })
    }

    fn __mod__(lhs: RGB, rhs: f64) -> PyResult<RGB> {
        let f = rhs / 100.;
        Ok(RGB {
            r: lhs.r * f,
            g: lhs.g * f,
            b: lhs.b + f,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Iterator {
    idx: usize,
    contents: vec::Vec<f64>,
}

impl Iterator {
    pub fn new(v: vec::Vec<f64>) -> Self {
        Self {
            idx: 0,
            contents: v,
        }
    }
}

#[pyproto]
impl PyIterProtocol for Iterator {
    fn __next__(mut item: PyRefMut<Iterator>) -> PyResult<Option<f64>> {
        if item.idx < item.contents.len() {
            let res = item.contents[item.idx];
            item.idx += 1;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

#[pyproto]
impl PyIterProtocol for RGB {
    fn __iter__(item: PyRefMut<RGB>) -> PyResult<Iterator> {
        Ok(item.into_iter())
    }
}

#[pyproto]
impl PyObjectProtocol for RGB {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!("{{{}, {}, {}}}", self.r, self.g, self.b))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!("RGB{{{}, {}, {}}}", self.r, self.g, self.b))
    }
}
