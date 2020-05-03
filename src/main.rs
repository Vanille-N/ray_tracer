#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;
extern crate rayon;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Range;
use std::process::Command;

mod camera;
mod composite_axes;
mod composite_cradle;
mod composite_die;
mod composite_molecules;
mod composite_erlenmeyer;
mod hitable;
mod primitives;
mod ray;
mod rgb;
mod vec3;

use camera::Camera;
use composite_axes::*;
use composite_cradle::*;
use composite_die::*;
use composite_molecules::*;
use composite_erlenmeyer::*;
use hitable::*;
use primitives::*;
use rgb::*;
use vec3::Vec3;

const EPSILON: f64 = 0.000_000_1;

const SYS: &str = "linux";

#[allow(unused_variables)]
fn main() {
    let cfg = build_world();
    let nb_cores = 5;
    let mut writers = Vec::new();
    for id in 0..nb_cores {
        //let out = File::create(&format!(".out{}.txt", idx)).unwrap();
        let out = BufWriter::new(File::create(&format!(".out{}.txt", id)).unwrap());
        let rng = (id * cfg.hgt / nb_cores)..((id + 1) * cfg.hgt / nb_cores);
        writers.push(Writer {
            id: id as u8,
            stdout: out,
            rng,
            cam: cfg.cam.clone(),
            world: cfg.world.clone(),
            sky: cfg.sky.clone(),
        });
    }

    if SYS == "linux" {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    writers.par_iter_mut().for_each(|w| {
        let color = &format!("\x1b[3{}m", w.id + 1);
        let ni = cfg.hgt as f64;
        let nj = cfg.wth as f64;
        for i in w.rng.clone().rev() {
            if SYS == "linux" {
                if i * 100 % cfg.hgt == 0 {
                    let load = 100 - i * 100 / cfg.hgt;
                    if load % 2 == 0 {
                        eprint!("\x1b[2B\x1b[{}C{}█\x1b[3A\n", load / 2, color);
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
                    let v = (i + rand::random::<f64>()) / ni;
                    let u = (j + rand::random::<f64>()) / nj;
                    let r = cfg.cam.get_ray(u, v);
                    c += hitable::color(&r, &w.world, 0, &w.sky);
                }
                write!(w.stdout, "{}", c / cfg.iter as f64).unwrap();
            }
            writeln!(w.stdout).unwrap();
        }
        w.stdout.flush().unwrap();
    });
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

#[derive(Clone)]
struct Writer<W: Write> {
    id: u8,
    stdout: W,
    rng: Range<usize>,
    cam: Camera,
    world: World,
    sky: Sky,
}

fn build_world() -> Cfg {
    let wth = 2000; // width in pixels
    let hgt = 1000; // height in pixels
    let iter = 100; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3::new(0.0, 1.0, 0.0), // target
        180.0,                     // angle (degrees)
        30.0,                     // rise (degrees)
        0.5,                      // distance (meters),
        30.0,                      // tilt (degrees)
        90.0,                     // aperture (degrees)
        wth as f64 / hgt as f64,    // aspect ratio
    );
    let sky = Sky::new("data/sky.ppm");
    let mut world = World::new();
    let ground = InfinitePlane {
        orig: Vec3::new(0.0, -0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        texture: Texture::Lambertian(RGB::new(0.8, 0.8, 0.1)),
    }
    .build()
    .wrap();

    let mirror1 = Parallelogram {
        a: Vec3::new(-1., 0., 1.),
        u: Vec3::new(2., 0., 0.),
        v: Vec3::new(0., 2., 0.),
        texture: Texture::Metal(RGB::new(0.95, 0.95, 0.95), 0.0),
    }
    .build()
    .wrap();
    let mirror2 = Parallelogram {
        a: Vec3::new(-1., 0., -1.),
        u: Vec3::new(2., 0., -0.05),
        v: Vec3::new(0., 2., 0.1),
        texture: Texture::Metal(RGB::new(0.95, 0.95, 0.95), 0.0),
    }
    .build()
    .wrap();


    let ball1 = Sphere {
        center: Vec3::new(0., 0.9, 0.),
        radius: 0.2,
        texture: Texture::Metal(RGB::new(0.6, 0.6, 1.), 0.),
    }
    .build()
    .wrap();

    let ball2 = Sphere {
        center: Vec3::new(0.3, 1.3, 0.5),
        radius: 0.15,
        texture: Texture::Lambertian(RGB::new(0., 0.2, 0.)),
    }
    .build()
    .wrap();

    let ball3 = Sphere {
        center: Vec3::new(-0.4, 1., -0.3),
        radius: 0.1,
        texture: Texture::Lambertian(RGB::new(0.7, 0., 0.)),
    }
    .build()
    .wrap();

    let ball4 = Sphere {
        center: Vec3::new(0.6, 0.5, -0.5),
        radius: 0.2,
        texture: Texture::Metal(RGB::new(0.3, 0., 0.7), 0.),
    }
    .build()
    .wrap();

    world.push(ball1);
    world.push(ball2);
    world.push(ball3);
    world.push(ball4);

    world.push(mirror1);
    world.push(mirror2);


    world.push(ground);

    Cfg { hgt, wth, iter, cam, world, sky }
}
