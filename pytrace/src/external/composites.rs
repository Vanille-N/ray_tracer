use crate::external::*;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
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

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(scale, /)"]
pub struct Axes {
    #[pyo3(get, set)]
    pub scale: f64,
}

#[pymethods]
impl Axes {
    #[new]
    pub fn new(scale: f64) -> Self {
        Self { scale }
    }

    #[text_signature = "($self, /)"]
    pub fn build(self) -> Prebuilt {
        Prebuilt {
            contents: Arc::new(self),
        }
    }
}

impl Develop for Axes {
    fn develop(&self) -> internal::Composite {
        composite::Axes { scale: self.scale }.build()
    }
}

#[pyproto]
impl PyObjectProtocol for Axes {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!("Axes({})", self.scale))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!("<Axes object with scale {}>", self.scale))
    }
}

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(position, rotation, size)"]
pub struct Cradle {
    #[pyo3(get, set)]
    pub position: Vec,
    #[pyo3(get, set)]
    pub rotation: f64,
    #[pyo3(get, set)]
    pub size: f64,
    pub amplitude: f64,
    pub time: f64,
}

#[pymethods]
impl Cradle {
    #[new]
    pub fn new(position: Vec, rotation: f64, size: f64) -> Self {
        Self {
            position,
            rotation,
            size,
            amplitude: 0.,
            time: 0.,
        }
    }

    #[text_signature = "($self, /)"]
    pub fn build(self) -> Prebuilt {
        Prebuilt {
            contents: Arc::new(self),
        }
    }

    #[text_signature = "($self, amount, /)"]
    pub fn raise_ball(&mut self, amount: f64) {
        self.amplitude = amount;
        self.time = std::f64::consts::PI / 2.;
    }

    #[text_signature = "($self, dt, /)"]
    pub fn tick(&mut self, dt: f64) {
        self.time += dt;
    }

    #[text_signature = "($self, t, /)"]
    pub fn set_time(&mut self, t: f64) {
        self.time = t;
    }
}

impl Cradle {
    fn calc_balls(&self) -> [f64; 5] {
        let c = (self.time * 2. * std::f64::consts::PI).sin() * self.amplitude;
        if c < 0. {
            [c, 0., 0., 0., 0.]
        } else {
            [0., 0., 0., 0., c]
        }
    }
}

impl Develop for Cradle {
    fn develop(&self) -> internal::Composite {
        composite::NewtonCradle {
            a: self.position.to_internal(),
            angle: self.rotation,
            size: self.size,
            pos: Some(self.calc_balls()),
        }
        .build()
    }
}

#[pyproto]
impl PyObjectProtocol for Cradle {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!(
            "Cradle({}, {})",
            repr!(self.position),
            self.size
        ))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!(
            "<Cradle object at {} with size {}>",
            repr!(self.position),
            self.size
        ))
    }
}

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(position, rotation, size)"]
pub struct Die {
    #[pyo3(get, set)]
    pub position: Vec,
    #[pyo3(get, set)]
    pub direction: Vec,
    #[pyo3(get, set)]
    pub rotation: f64,
    pub side_texture: Texture,
    pub edge_texture: Texture,
    pub dot_texture: Texture,
}

#[pymethods]
impl Die {
    #[new]
    pub fn new(position: Vec, direction: Vec, rotation: f64, side_texture: Texture, edge_texture: Texture, dot_texture: Texture) -> Self {
        Self {
            position, direction, rotation, side_texture, edge_texture, dot_texture,
        }
    }

    #[text_signature = "($self, /)"]
    pub fn build(self) -> Prebuilt {
        Prebuilt {
            contents: Arc::new(self),
        }
    }
}

impl Develop for Die {
    fn develop(&self) -> internal::Composite {
        composite::Die {
            a: self.position.to_internal(),
            up: self.direction.to_internal(),
            rot: self.rotation,
            side_texture: self.side_texture.to_internal(),
            edge_texture: self.edge_texture.to_internal(),
            dot_texture: self.dot_texture.to_internal(),
        }
        .build()
    }
}

#[pyproto]
impl PyObjectProtocol for Die {
    fn __repr__(self) -> PyResult<String> {
        Ok(format!(
            "Die({}, {}, {}) with textures [{} {} {}]",
            repr!(self.position),
            repr!(self.direction),
            self.rotation,
            repr!(self.side_texture),
            repr!(self.edge_texture),
            repr!(self.dot_texture),
        ))
    }

    fn __str__(self) -> PyResult<String> {
        Ok(format!(
            "<Die object at {} facing {} ({})>",
            repr!(self.position),
            repr!(self.direction),
            self.rotation,
        ))
    }
}
