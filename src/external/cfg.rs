use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::composite;
use crate::external::*;
use crate::internal;

#[pyclass]
pub struct Cfg {
    pub silent: bool,
    pub hgt: usize,
    pub wth: usize,
    pub iter: usize,
    pub cam: Option<Camera>,
    pub world: internal::World,
    pub sky: Option<Sky>,
}

pub struct Builder {
    pub name: String,
    pub silent: bool,
    pub hgt: usize,
    pub wth: usize,
    pub iter: usize,
    pub cam: internal::Camera,
    pub world: internal::World,
    pub sky: internal::Sky,
}

#[pymethods]
impl Cfg {
    #[new]
    pub fn new(wth: usize, hgt: usize, iter: usize) -> Self {
        Self {
            silent: false,
            hgt,
            wth,
            iter,
            cam: None,
            world: internal::World::new(),
            sky: None,
        }
    }

    #[text_signature = "($self)"]
    pub fn silence(&mut self) {
        self.silent = true;
    }

    #[text_signature = "($self, name)"]
    pub fn render(&self, name: String) {
        if let Some(mut cam) = self.cam {
            if cam.aspect < 0. {
                cam.aspect = self.wth as f64 / self.hgt as f64;
            }
            if let Some(sky) = &self.sky {
                crate::render(Builder {
                    name,
                    silent: self.silent,
                    hgt: self.hgt,
                    wth: self.wth,
                    iter: self.iter,
                    cam: cam.to_internal(),
                    world: self.world.clone(),
                    sky: sky.clone(),
                })
            } else {
                panic!("No sky provided")
            }
        } else {
            panic!("No camera provided")
        }
    }

    #[text_signature = "($self, color)"]
    pub fn set_background(&mut self, c: RGB) {
        self.world.background = Some(c.to_internal());
    }

    #[text_signature = "($self, sky)"]
    pub fn set_cam(&mut self, cam: Camera) {
        self.cam = Some(cam);
    }

    #[text_signature = "($self, sky)"]
    pub fn set_sky(&mut self, sky: Sky) {
        self.sky = Some(sky)
    }

    #[text_signature = "($self)"]
    pub fn populate(&mut self) {
        self.world.push_vec(
            composite::NewtonCradle {
                a: internal::Vec3(-0.5, 0., -0.5),
                angle: 0.,
                size: 1.,
            }
            .build(),
        );
        self.world.push(
            internal::InfinitePlane {
                orig: internal::Vec3(0., 0., 0.),
                normal: internal::Vec3(0., 1., 0.),
                texture: internal::Texture::Lambertian(internal::RGB(0.5, 0.5, 0.5)),
            }
            .build()
            .wrap(),
        );
    }

    #[text_signature = "($self, object)"]
    pub fn add_obj(&mut self, obj: Primitive) {
        self.world.push(obj.extract());
    }
}
