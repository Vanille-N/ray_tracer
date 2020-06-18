use glob::glob;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use std::process::Command;

use crate::external::*;
use pytrace_core::internal;
use pytrace_core::render::*;
use ctrlc;

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
    #[pyo3(get, set)]
    nbsync: usize,
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
        ctrlc::set_handler(|| std::process::exit(2)).unwrap_or_else(|_| eprintln!("\x1b[33;1mWarning: you have multiple configurations at the same time.\x1b[0m
That's ok, but don't mix them up."));
        Self {
            silent: false,
            hgt,
            wth,
            iter,
            cam: None,
            world: internal::World::new(),
            sky: None,
            mov: None,
            nbsync: 5,
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
                    nbsync: self.nbsync,
                })
            } else {
                panic!("No sky provided")
            }
        } else {
            panic!("No camera provided")
        }
    }

    #[text_signature = "($self, r, g, b, /)"]
    pub fn set_background(&mut self, r: f64, g: f64, b: f64) {
        self.world.background = Some(internal::RGB(r, g, b));
        self.refresh();
    }

    #[text_signature = "($self, /)"]
    pub fn true_background(&mut self) {
        self.world.background = None;
        self.refresh();
    }

    #[text_signature = "($self, camera, /)"]
    pub fn set_cam(&mut self, cam: Camera) {
        self.cam = Some(cam);
        self.refresh();
    }

    #[text_signature = "($self, sky, /)"]
    pub fn set_sky(&mut self, sky: Sky) {
        self.sky = Some(sky);
        self.refresh();
    }

    #[text_signature = "($self, object, /)"]
    pub fn populate(&mut self, object: Prebuilt) {
        self.world.push_vec(object.extract());
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

    #[text_signature = "($self, /)"]
    pub fn end_movie(&mut self) {
        if let Some(m) = &self.mov {
            Command::new("rm")
                .arg(&format!("{}.avi", &m.name))
                .status()
                .expect("No file to remove");
            let e = Command::new("ffmpeg")
                .arg("-pattern_type")
                .arg("sequence")
                .arg("-framerate")
                .arg("25")
                .arg("-i")
                .arg(&format!("img-{}-%d.ppm", &m.name))
                .arg("-vcodec")
                .arg("libx264")
                .arg(&format!("{}.avi", &m.name))
                .status()
                .expect("Failed to create movie");
            if e.success() {
                println!("Done creating movie, cleanup files");
                for f in
                    glob(&format!("img-{}-*.ppm", &m.name)).expect("Could not read glob pattern")
                {
                    Command::new("rm")
                        .arg(&format!("{}", f.unwrap().display()))
                        .status()
                        .expect("Failed to remove");
                }
                self.mov = None;
            }
        }
    }
}

impl Cfg {
    fn refresh(&mut self) {
        if let Some(m) = &mut self.mov {
            m.modif = true;
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Cfg {
    fn __str__(self) -> PyResult<String> {
        Ok(format!(
            "Cfg {{
    silent: {},
    hgt:    {},
    wth:    {},
    iter:   {},
    nbsync: {},
}}",
            self.silent, self.hgt, self.wth, self.iter, self.nbsync,
        ))
    }
}
