#![allow(dead_code)]
#![allow(unused_imports)]

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Range;
use std::process::Command;
use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;

mod camera;
mod composite_axes;
mod composite_cradle;
mod composite_die;
mod composite_flasks;
mod composite_molecules;
mod hitable;
mod primitives;
mod ray;
mod rgb;
mod sky;
mod vec3;
mod world;

mod pycfg;
mod pycamera;
mod pyvec3;

mod internal {
    pub use crate::camera::Camera;
    pub use crate::composite_axes::Axes;
    pub use crate::composite_cradle::NewtonCradle;
    pub use crate::composite_die::Die;
    pub use crate::composite_flasks::Flask;
    pub use crate::composite_molecules::Molecule;
    pub use crate::hitable::*;
    pub use crate::primitives::*;
    pub use crate::rgb::*;
    pub use crate::world::World;
    pub use crate::vec3::Vec3;
    pub use crate::sky::Sky;
}

mod external {
    pub use crate::pycfg::*;
    pub use crate::pycamera::*;
    pub use crate::pyvec3::*;
    pub use crate::sky::*;
}


const EPSILON: f64 = 0.000_000_1;


fn render(build: external::Builder) {
    let build = Arc::new(build);
    let nb_cores = 5;
    if !build.silent {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    let pool = ThreadPool::new(nb_cores);
    let barrier = Arc::new(Barrier::new(nb_cores + 1));
    for id in 0..nb_cores {
        let mut stdout = BufWriter::new(File::create(&format!(".out{}.txt", id)).unwrap());
        let rng = (id * build.hgt / nb_cores)..((id + 1) * build.hgt / nb_cores);
        let barrier = barrier.clone();
        let build = build.clone();
        pool.execute(move || {
            let color = &format!("\x1b[3{}m", id + 1);
            let ni = build.hgt as f64;
            let nj = build.wth as f64;
            for i in rng.rev() {
                if !build.silent {
                    if i * 100 % build.hgt == 0 {
                        let load = 100 - i * 100 / build.hgt;
                        if load % 2 == 0 {
                            eprint!("\x1b[{}C{}█\x1b[1A\n", load / 2, color);
                        }
                    }
                }

                for j in 0..build.wth {
                    let mut c = internal::BLACK;
                    let i = i as f64;
                    let j = j as f64;
                    for _ in 0..build.iter {
                        let vfrac = (i + rand::random::<f64>()) / ni;
                        let hfrac = (j + rand::random::<f64>()) / nj;
                        let r = build.cam.get_ray(hfrac, vfrac);
                        c += world::calc_color(&r, &build.world, &build.sky);
                    }
                    write!(stdout, "{}", c / build.iter as f64).unwrap();
                }
                writeln!(stdout).unwrap();
            }
            stdout.flush().unwrap();
            barrier.wait();
        });
    }
    barrier.wait();
    if !build.silent {
        eprint!("\n\n\n\x1b[0m");
    }
    let mut f = File::create("img.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", build.wth, build.hgt).unwrap();
    for idx in (0..nb_cores).rev() {
        let output = Command::new("cat")
            .arg(&format!(".out{}.txt", idx))
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
        Command::new("rm")
            .arg(&format!(".out{}.txt", idx))
            .status()
            .expect("Failed to cleanup directory");
    }
}

#[pymodule]
fn pytrace(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<external::Cfg>().unwrap();
    m.add_class::<external::Camera>().unwrap();
    m.add_class::<external::Vec3>().unwrap();
    m.add_class::<external::Sky>().unwrap();
    Ok(())
}
