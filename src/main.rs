#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;
extern crate rayon;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
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

const CFG: &str = "linux";

#[allow(unused_variables)]
fn main() {
    let (ni, nj, ns, cam, w, sky) = build_world();
    let nb_cores = 5;
    let mut writers = Vec::new();
    for idx in (0..nb_cores).rev() {
        //let out = File::create(&format!(".out{}.txt", idx)).unwrap();
        let out = BufWriter::new(File::create(&format!(".out{}.txt", idx)).unwrap());
        let ni_rng = (idx * ni / nb_cores)..((idx + 1) * ni / nb_cores);
        writers.push((idx, out, ni_rng, cam.clone(), w.clone(), sky.clone()));
    }

    if CFG == "linux" {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    writers.par_iter_mut().for_each(|(id, f, range, cam, w, sky)| {
        let color = &format!("\x1b[3{}m", *id + 1);
        for i in range.rev() {
            if CFG == "linux" {
                if i * 100 % ni == 0 {
                    let load = 100 - i * 100 / ni;
                    if load % 2 == 0 {
                        eprint!("\x1b[2B\x1b[{}C{}█\x1b[3A\n", load / 2, color);
                    }
                }
            } else if i * 100 % ni == 0 {
                let load = 100 - i * 100 / ni;
                eprintln!("{}%", load);
            }

            for j in 0..nj {
                let mut c = RGB::new(0., 0., 0.);
                let i = i as f64;
                let j = j as f64;
                let ni = ni as f64;
                let nj = nj as f64;
                for _ in 0..ns {
                    let v = (i + rand::random::<f64>()) / ni;
                    let u = (j + rand::random::<f64>()) / nj;
                    let r = cam.get_ray(u, v);
                    c += hitable::color(&r, &w, 0, sky);
                }
                write!(f, "{}", c / ns as f64).unwrap();
                //f.write(format!("{}", c / ns as f64).as_bytes()).unwrap();
            }
            writeln!(f).unwrap();
            //f.write(b"\n").unwrap();
        }
        f.flush().unwrap();
    });
    print!("\n\n\n\x1b[0m");
    let mut f = File::create("img.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", nj, ni).unwrap();
    for idx in (0..nb_cores).rev() {
        let output = Command::new("cat")
            .arg(&format!(".out{}.txt", idx))
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    }
}

fn build_world() -> (i32, i32, i32, Camera, World, Sky) {
    let nj = 1000; // width in pixels
    let ni = 1000; // height in pixels
    let ns = 50; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3::new(0.0, 1.0, 0.0), // target
        180.0,                     // angle (degrees)
        30.0,                     // rise (degrees)
        7.0,                      // distance (meters),
        0.0,                      // tilt (degrees)
        40.0,                     // aperture (degrees)
        nj as f64 / ni as f64,    // aspect ratio
    );
    let sky = Sky::new("data/sky.ppm");
    let mut w = World::new();
    let ground = InfinitePlane {
        orig: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        texture: Texture::Lambertian(RGB::new(0.4, 0.4, 0.4)),
    }
    .build()
    .wrap();


    let ball = Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        texture: Texture::Metal(RGB::new(0.6, 0.6, 1.), 0.),
    }
    .build()
    .wrap();

    w.push(ball);

    w.push(ground);

    (ni, nj, ns, cam, w, sky)
}
