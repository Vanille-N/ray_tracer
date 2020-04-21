#![allow(dead_code)]

extern crate rand;
extern crate rayon;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod camera;
mod composite_axes;
mod composite_cradle;
mod composite_die;
mod composite_molecules;
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
use hitable::*;
use primitives::*;
use rgb::*;
use vec3::Vec3;

const EPSILON: f64 = 0.000_000_1;

const CFG: &str = "linux";

#[allow(unused_variables)]
fn main() {
    let (ni, nj, ns, cam, w) = build_world();
    let nb_cores = 5;
    let mut writers = Vec::new();
    for idx in (0..nb_cores).rev() {
        let out = File::create(&format!(".out{}.txt", idx)).unwrap();
        let ni_rng = (idx * ni / nb_cores)..((idx + 1) * ni / nb_cores);
        writers.push((idx, out, ni_rng, cam.clone(), w.clone()));
    }

    if CFG == "linux" {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    writers.par_iter_mut().for_each(|(id, f, range, cam, w)| {
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
                    c += hitable::color(&r, &w, 0);
                }
                write!(f, "{}", c / ns as f64).unwrap();
            }
            writeln!(f).unwrap();
        }
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

fn build_world() -> (i32, i32, i32, Camera, World) {
    let nj = 2000; // width in pixels
    let ni = 2000; // height in pixels
    let ns = 100; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3::new(0.0, 1.0, 0.0), // target
        30.0,                     // angle (degrees)
        30.0,                     // rise (degrees)
        7.0,                      // distance (meters),
        0.0,                      // tilt (degrees)
        40.0,                     // aperture (degrees)
        nj as f64 / ni as f64,    // aspect ratio
    );
    let mut w = World::new();
    let ground = InfinitePlane {
        orig: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        texture: Texture::Lambertian(RGB::new(0.2, 0.2, 0.2)),
    }
    .build()
    .wrap();

    let red1 = Texture::Metal(RED % 90 + BLUE % 10 + GREEN % 10, 0.05);
    let red2 = Texture::Metal(RED % 70 + BLUE % 5 + GREEN % 5, 0.05);
    let blk = Texture::Metal(RED % 5 + BLUE % 5 + GREEN % 5, 0.05);
    let grn1 = Texture::Metal(RED % 5 + BLUE % 5 + GREEN % 50, 0.05);
    let grn2 = Texture::Metal(RED % 2 + BLUE % 2 + GREEN % 30, 0.05);
    let ylw = Texture::Metal(YELLOW % 40, 0.05);
    let blu1 = Texture::Metal(RED % 10 + BLUE % 90 + GREEN % 10, 0.05);
    let blu2 = Texture::Metal(RED % 5 + BLUE % 70 + GREEN % 5, 0.05);
    let wht = Texture::Metal(WHITE % 80, 0.05);

    let die1 = Die {
        a: Vec3::new(-1.0, 0.0, -1.0),
        u: Vec3::new(0.0, 0.0, 2.0),
        v: Vec3::new(2.0, 0.0, 0.0),
        w: Vec3::new(0.0, 2.0, 0.0),
        dot_texture: blk,
        side_texture: red1,
        edge_texture: red2,
    }
    .build();
    let die2 = Die {
        a: Vec3::new(2.5, 0.0, -1.2),
        u: Vec3::new(3.0, 0.0, 1.0),
        v: Vec3::new(-2.0, 0.0, 0.0),
        w: Vec3::new(0.0, 2.0, 0.0),
        dot_texture: ylw,
        side_texture: grn1,
        edge_texture: grn2,
    }
    .build();
    let die3 = Die {
        a: Vec3::new(-4.0, 2.0, -3.0),
        u: Vec3::new(-1.0, -3.0, 0.0),
        v: Vec3::new(-2.0, -2.0, 0.0),
        w: Vec3::new(2.0, 0.0, 0.0),
        dot_texture: wht,
        side_texture: blu1,
        edge_texture: blu2,
    }
    .build();

    w.push_vec(die1);
    w.push_vec(die2);
    w.push_vec(die3);
    w.push(ground);

    (ni, nj, ns, cam, w)
}
