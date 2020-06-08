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

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(scale, /)"]
pub struct Axes {
    #[pyo3(get, set)] pub scale: f64,
}

#[pymethods]
impl Axes {
    #[new]
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
        }
    }

    #[text_signature = "($self, /)"]
    pub fn build(self) -> Prebuilt {
        Prebuilt {
            contents: Arc::new(self)
        }
    }
}

impl Develop for Axes {
    fn develop(&self) -> internal::Composite {
        composite::Axes {
            scale: self.scale,
        }.build()
    }
}

#[pyclass]
#[derive(Copy, Clone)]
#[text_signature = "(position, rotation, size)"]
pub struct Cradle {
    #[pyo3(get, set)] pub position: Vec,
    #[pyo3(get, set)] pub rotation: f64,
    #[pyo3(get, set)] pub size: f64,
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
            contents: Arc::new(self)
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
        }.build()
    }
}
