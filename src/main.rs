#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;
extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::{Arc, Barrier};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Range;
use std::process::Command;

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

use camera::Camera;
use composite_axes::Axes;
use composite_cradle::NewtonCradle;
use composite_die::Die;
use composite_flasks::Flask;
use composite_molecules::Molecule;
use hitable::*;
use primitives::*;
use rgb::*;
use sky::Sky;
use vec3::Vec3;
use world::World;

const EPSILON: f64 = 0.000_000_1;

const SYS: &str = "linux";

fn main() {
    let cfg = build_world();
    let cfg = Arc::new(cfg);
    let nb_cores = 4;
    if SYS == "linux" {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    let pool = ThreadPool::new(nb_cores);
    let barrier = Arc::new(Barrier::new(nb_cores + 1));
    for id in 0..nb_cores {
        // Clone essential info
        let mut stdout = BufWriter::new(File::create(&format!(".out{}.txt", id)).unwrap());
        let rng = (id * cfg.hgt / nb_cores)..((id + 1) * cfg.hgt / nb_cores);
        let barrier = barrier.clone();
        let cfg = cfg.clone();
        pool.execute(move || {
            let color = &format!("\x1b[3{}m", id + 1);
            let ni = cfg.hgt as f64;
            let nj = cfg.wth as f64;
            for i in rng.rev() {
                if SYS == "linux" {
                    if i * 100 % cfg.hgt == 0 {
                        let load = 100 - i * 100 / cfg.hgt;
                        if load % 2 == 0 {
                            eprint!("\x1b[{}C{}█\x1b[1A\n", load / 2, color);
                        }
                    }
                } else if i * 100 % cfg.hgt == 0 {
                    let load = 100 - i * 100 / cfg.hgt;
                    eprintln!("{}%", load);
                }

                for j in 0..cfg.wth {
                    let mut c = BLACK;
                    let i = i as f64;
                    let j = j as f64;
                    for _ in 0..cfg.iter {
                        let vfrac = (i + rand::random::<f64>()) / ni;
                        let hfrac = (j + rand::random::<f64>()) / nj;
                        let r = cfg.cam.get_ray(hfrac, vfrac);
                        c += world::color(&r, &cfg.world, 0, &cfg.sky);
                    }
                    write!(stdout, "{}", c / cfg.iter as f64).unwrap();
                }
                writeln!(stdout).unwrap();
            }
            stdout.flush().unwrap();
            barrier.wait();
        });
    }
    barrier.wait();
    if SYS == "linux" {
        eprint!("\n\n\n\x1b[0m");
    }
    let mut f = File::create("img.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", cfg.wth, cfg.hgt).unwrap();
    for idx in (0..nb_cores).rev() {
        let output = Command::new("cat")
            .arg(&format!(".out{}.txt", idx))
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    }
}

struct Cfg {
    hgt: usize,
    wth: usize,
    iter: usize,
    cam: Camera,
    world: World,
    sky: Sky,
}

fn build_world() -> Cfg {
    let wth = 500; // width in pixels
    let hgt = 500; // height in pixels
    let iter = 100; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3(0.0, 1.0, 0.0),     // target
        0.0,                     // angle (degrees)
        60.0,                    // rise (degrees)
        3.0,                     // distance (meters),
        0.0,                     // tilt (degrees)
        90.0,                    // aperture (degrees)
        wth as f64 / hgt as f64, // aspect ratio
    );
    let sky = Sky::blank();
    let mut world = World::new();
    let ground = InfinitePlane {
        orig: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 1.0, 0.0),
        texture: Texture::Lambertian(RGB(0.5, 0.5, 0.6)),
    }
    .build()
    .wrap();

    let erlen = Flask {
        a: Vec3(0.0, 0.0, 0.0),
        size: 1.,
        color: RGB(0.5, 0.8, 1.0),
    }
    .florence();

    world.push(ground);
    world.push_vec(erlen);

    Cfg {
        hgt,
        wth,
        iter,
        cam,
        world,
        sky,
    }
}
