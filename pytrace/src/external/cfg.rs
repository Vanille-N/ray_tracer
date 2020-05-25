use pyo3::prelude::*;

use crate::external::*;
use libtrace::composite;
use libtrace::internal;
use libtrace::render::*;

#[pyclass]
#[text_signature = "(wth, hgt, iter, /)"]
pub struct Cfg {
    pub silent: bool,
    #[pyo3(get, set)]
    pub hgt: usize,
    #[pyo3(get, set)]
    pub wth: usize,
    #[pyo3(get, set)]
    pub iter: usize,
    pub cam: Option<Camera>,
    pub world: internal::World,
    pub sky: Option<Sky>,
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

    #[text_signature = "($self, /)"]
    pub fn silence(&mut self) {
        self.silent = true;
    }

    #[text_signature = "($self, name, /)"]
    pub fn render(&self, name: String) {
        if let Some(mut cam) = self.cam {
            if cam.aspect < 0. {
                cam.aspect = self.wth as f64 / self.hgt as f64;
            }
            if let Some(sky) = &self.sky {
                render(Builder {
                    name,
                    silent: self.silent,
                    hgt: self.hgt,
                    wth: self.wth,
                    iter: self.iter,
                    cam: cam.to_internal(),
                    world: self.world.clone(),
                    sky: sky.to_internal(),
                })
            } else {
                panic!("No sky provided")
            }
        } else {
            panic!("No camera provided")
        }
    }

    #[text_signature = "($self, color, /)"]
    pub fn set_background(&mut self, color: RGB) {
        self.world.background = Some(color.to_internal());
    }

    #[text_signature = "($self)"]
    pub fn true_background(&mut self) {
        self.world.background = None;
    }

    #[text_signature = "($self, sky, /)"]
    pub fn set_cam(&mut self, cam: Camera) {
        self.cam = Some(cam);
    }

    #[text_signature = "($self, sky, /)"]
    pub fn set_sky(&mut self, sky: Sky) {
        self.sky = Some(sky)
    }

    #[text_signature = "($self, /)"]
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

    #[text_signature = "($self, /)"]
    pub fn clear(&mut self) {
        self.world.clear();
    }

    #[text_signature = "($self, object, /)"]
    pub fn add_obj(&mut self, object: Construct) {
        self.world.push_vec(object.contents.canonical());
    }
}
