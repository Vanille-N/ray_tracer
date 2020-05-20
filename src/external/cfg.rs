use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::internal;
use crate::external;

#[pyclass]
pub struct Cfg {
    pub silent: bool,
    pub hgt: usize,
    pub wth: usize,
    pub iter: usize,
    pub cam: Option<external::Camera>,
    pub world: internal::World,
    pub sky: Option<external::Sky>,
}

pub struct Builder {
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

    #[text_signature = "($self)"]
    pub fn render(&self) {
        if let Some(mut cam) = self.cam {
            if cam.aspect < 0. {
                cam.aspect = self.wth as f64 / self.hgt as f64;
            }
            if let Some(sky) = &self.sky {
                crate::render(Builder {
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

    #[text_signature = "($self, sky)"]
    pub fn add_cam(&mut self, cam: external::Camera) {
        self.cam = Some(cam);
    }

    #[text_signature = "($self, sky)"]
    pub fn add_sky(&mut self, sky: external::Sky) {
        self.sky = Some(sky)
    }

    pub fn add_obj(&mut self) {
        let t2 = internal::Texture::Lambertian(internal::RGB(0.2, 0.8, 0.3));

        let x = internal::Vec3(10., 0., 0.);
        let y = internal::Vec3(0., 10., 0.);
        let z = internal::Vec3(0., 0., 30.);

        let ecirc = internal::Cylinder {
            center1: x * 7.5 + y * 1.5,
            center2: x * 7.5 + y * 1.5 + z,
            radius: 15.,
            texture: t2,
        }
        .build()
        .remove(
            internal::Cylinder {
                center1: x * 7.5 + y * 1.5 - z * 0.1,
                center2: x * 7.5 + y * 1.5 + z * 1.1,
                radius: 10.,
                texture: t2,
            }
            .build(),
        )
        .remove(
            internal::Rhomboid {
                a: x * 7.5 + y * 1.5 - z * 0.1,
                u: x * 10. + y * 5.,
                v: x * 10. - y * 5.,
                w: z * 1.2,
                texture: t2,
            }
            .build(),
        );

        let ehbar = internal::Rhomboid {
            a: x * 6.5 + y * 1.,
            u: x * 2. + y * 1.,
            v: y * 0.57,
            w: z,
            texture: t2,
        }
        .build()
        .wrap();
        self.world.push(ecirc);
        self.world.push(ehbar);
    }
}
