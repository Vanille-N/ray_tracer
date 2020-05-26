use pyo3::prelude::*;
use std::process::Command;

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
    cam: Option<Camera>,
    world: internal::World,
    sky: Option<Sky>,
    mov: Option<MovieCfg>,
}

struct MovieCfg {
    name: String,
    cnt: usize,
    modif: bool,
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
            mov: None,
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
        self.refresh();
    }

    #[text_signature = "($self)"]
    pub fn true_background(&mut self) {
        self.world.background = None;
        self.refresh();
    }

    #[text_signature = "($self, sky, /)"]
    pub fn set_cam(&mut self, cam: Camera) {
        self.cam = Some(cam);
        self.refresh();
    }

    #[text_signature = "($self, sky, /)"]
    pub fn set_sky(&mut self, sky: Sky) {
        self.sky = Some(sky);
        self.refresh();
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
        self.refresh();
    }

    #[text_signature = "($self, /)"]
    pub fn clear(&mut self) {
        self.world.clear();
        self.refresh();
    }

    #[text_signature = "($self, object, /)"]
    pub fn add_obj(&mut self, object: Construct) {
        self.world.push_vec(object.contents.canonical());
        self.refresh();
    }

    #[text_signature = "($self, name, /)"]
    pub fn start_movie(&mut self, name: String) {
        self.mov = Some(MovieCfg {
            name,
            cnt: 0,
            modif: true,
        });
    }

    #[text_signature = "($self, name, /)"]
    pub fn frame(&mut self) {
        if let Some(m) = &self.mov {
            println!("Creating frame {}", m.cnt);
            if m.modif {
                self.render(format!("{}-{}", &m.name, m.cnt));
            } else {
                Command::new("cp")
                    .arg(&format!("img-{}-{}.ppm", &m.name, m.cnt - 1))
                    .arg(&format!("img-{}-{}.ppm", &m.name, m.cnt))
                    .status()
                    .expect("Could not copy previous image");
            }
        } else {
            panic!("No movie configured");
        }
        if let Some(m) = &mut self.mov {
            m.cnt += 1;
            m.modif = false;
        }
    }
}
