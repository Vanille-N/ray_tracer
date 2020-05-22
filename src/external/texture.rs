use crate::internal;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
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
}

pub const RED: RGB = RGB {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
pub const DKRED: RGB = RGB {
    r: 0.5,
    g: 0.0,
    b: 0.0,
};
pub const LTRED: RGB = RGB {
    r: 1.0,
    g: 0.5,
    b: 0.5,
};
pub const BLUE: RGB = RGB {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};
pub const DKBLUE: RGB = RGB {
    r: 0.0,
    g: 0.0,
    b: 0.5,
};
pub const LTBLUE: RGB = RGB {
    r: 0.3,
    g: 0.6,
    b: 1.0,
};
pub const CYAN: RGB = RGB {
    r: 0.0,
    g: 1.0,
    b: 1.0,
};
pub const GREEN: RGB = RGB {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
pub const DKGREEN: RGB = RGB {
    r: 0.0,
    g: 0.5,
    b: 0.0,
};
pub const LTGREEN: RGB = RGB {
    r: 0.7,
    g: 1.0,
    b: 0.0,
};
pub const PURPLE: RGB = RGB {
    r: 0.7,
    g: 0.0,
    b: 0.0,
};
pub const MAGENTA: RGB = RGB {
    r: 1.0,
    g: 0.0,
    b: 1.0,
};
pub const YELLOW: RGB = RGB {
    r: 1.0,
    g: 1.0,
    b: 0.0,
};
pub const BROWN: RGB = RGB {
    r: 0.3,
    g: 0.2,
    b: 0.0,
};
pub const ORANGE: RGB = RGB {
    r: 1.0,
    g: 0.4,
    b: 0.0,
};
pub const TURQUOISE: RGB = RGB {
    r: 0.0,
    g: 0.9,
    b: 0.6,
};
pub const BLACK: RGB = RGB {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const WHITE: RGB = RGB {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
pub const GREY: RGB = RGB {
    r: 0.5,
    g: 0.5,
    b: 0.5,
};
pub const DKGREY: RGB = RGB {
    r: 0.2,
    g: 0.2,
    b: 0.2,
};
pub const LTGREY: RGB = RGB {
    r: 0.8,
    g: 0.8,
    b: 0.8,
};

#[pyclass]
#[derive(Clone, Copy)]
pub struct Texture {
    contents: internal::Texture,
}

#[pymethods]
impl Texture {
    #[staticmethod]
    pub fn lambertian(c: RGB) -> Self {
        Self {
            contents: internal::Texture::Lambertian(c.to_internal()),
        }
    }

    #[staticmethod]
    pub fn metal(c: RGB, fuzzy: f64) -> Self {
        Self {
            contents: internal::Texture::Metal(c.to_internal(), fuzzy),
        }
    }

    #[staticmethod]
    pub fn light(c: RGB) -> Self {
        Self {
            contents: internal::Texture::Light(c.to_internal()),
        }
    }

    #[staticmethod]
    pub fn dielectric(c: RGB, idx: f64) -> Self {
        Self {
            contents: internal::Texture::Dielectric(c.to_internal(), idx),
        }
    }
}

impl Texture {
    pub fn to_internal(self) -> internal::Texture {
        self.contents
    }
}
