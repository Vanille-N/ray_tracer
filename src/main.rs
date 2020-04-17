#![allow(dead_code)]

extern crate rand;
extern crate rayon;
//extern crate cmd_lib;
//use rand::Rng;
//use cmd_lib::run_cmd;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use rayon::prelude::*;

mod ray;
mod rgb;
mod vec3;
mod hitable;
mod camera;
mod primitives;
mod composite_cradle;
mod composite_molecules;
mod composite_axes;

use vec3::Vec3;
use rgb::RGB;
use hitable::*;
use camera::Camera;
use primitives::*;
use composite_cradle::*;
use composite_molecules::*;
use composite_axes::*;

const EPSILON: f64 = 0.000001;

#[allow(unused_variables)]
fn main() {
    //let mut rng = rand::thread_rng();
    let nj = 200; // width in pixels
    let ni = 200; // height in pixels
    let ns = 100; // number of samples per pixel
    // let cam = Camera::new_absolute(
    //     Vec3::new(-2.0, 3.0, -6.0), // eye
    //     Vec3::new(0.0, 0.5, 0.0), // target
    //     Vec3::new(0.0, 1.0, 0.0), // up
    //     40.0, // aperture (degrees)
    //     nj as f64 / ni as f64, // aspect ratio
    // );
    let cam = Camera::new_relative(
        Vec3::new(0.0, 0.3, 0.25), // target
        90.0, // angle (degrees)
        45.0, // rise (degrees)
        1.5, // distance (meters),
        0.0, // tilt (degrees)
        40.0, // aperture (degrees)
        nj as f64 / ni as f64, // aspect ratio
    );
    let mut w = World::new();
    let ground = InfinitePlane {
        orig: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        texture: Texture::Metal(RGB::new(0.2, 0.2, 0.2), 0.1),
    }.build().wrap();
    let axes = Axes(2.0).build().wrap();
    let sun = Sphere {
        center: Vec3::new(-100.0, 100.0, -100.0),
        radius: 30.0,
        texture: Texture::Light(RGB::new(3.0, 3.0, 3.0)),
    }.build().wrap();
    let cradle = NewtonCradle {
        a: Vec3::new(-0.5, 0.0, -0.5),
        u: Vec3::new(0.0, 0.0, 1.0),
        v: Vec3::new(1.0, 0.0, 0.0),
        w: Vec3::new(0.0, 1.0, 0.0),
    }.build().wrap();
    // let cyc = Molecule {
    //     c_ref: Vec3::new(-5.0, 0.7, 17.0),
    //     up: Vec3::new(0.3, 0.3, 0.0),
    //     fwd: Vec3::new(-1.0, 0.5, 1.0),
    // }.cyclohexanol().build().wrap();
    // let water = Molecule {
    //     c_ref: Vec3::new(0.0, 0.0, 0.0),
    //     up: Vec3::new(0.0, 1.0, 0.0),
    //     fwd: Vec3::new(-1.0, 0.5, 1.0),
    // }.water().build().wrap();
    // let methane = Molecule {
    //     c_ref: Vec3::new(6.0, -5.0, 0.0),
    //     up: Vec3::new(0.1, 0.3, 0.0),
    //     fwd: Vec3::new(-1.0, 0.5, 1.0),
    // }.methane().build().wrap();
    // let ethanol = Molecule {
    //     c_ref: Vec3::new(-10.0, 1.0, -43.0),
    //     up: Vec3::new(0.1, 0.3, 0.0),
    //     fwd: Vec3::new(-1.0, 0.5, 1.0),
    // }.ethanol().build().wrap();
    // let obj1 = Rhombus {
    //     a: Vec3::new(0.0, -1.0, 0.0),
    //     u: Vec3::new(0.0, 0.0, 1.0),
    //     v:  Vec3::new(2.0, 0.0, 0.5),
    //     w:  Vec3::new(0.0, 1.0, 0.0),
    //     texture: Texture::Dielectric(RGB::new(0.9, 0.9, 0.9), 1.5),
    // }.build().wrap();
    // let ball2 = Sphere {
    //     center: Vec3::new(1.0, 0.0, -5.0),
    //     radius: 0.5,
    //     texture: Texture::Metal(RGB::new(0.8, 0.6, 0.2), 0.0),
    // }.build().wrap();
    // let ball3 = Sphere {
    //     center: Vec3::new(-1.0, 0.0, 0.0),
    //     radius: 0.5,
    //     texture: Texture::Lambertian(RGB::new(0.9, 0.9, 0.1)),
    // }.build().wrap();
    let n2 = Molecule {
        c_ref: Vec3::new(0.0, 0.3, 0.0),
        up: Vec3::new(0.0, 0.0, 0.1),
        fwd: Vec3::new(0.0, 0.1, 0.0),
    }.dinitrogen().build().wrap();
    w.push(ground);
    w.push(n2);
    //w.push(water);
    //w.push(cyc);
    //w.push(methane);
    //w.push(ethanol);
    //w.push(cradle);
    w.push(sun);

    let nb_cores = 4;
    let mut writers = Vec::new();
    for idx in (0..nb_cores).rev() {
        let out = File::create(&format!(".out{}.txt", idx)).unwrap();
        let ni_rng = (idx*ni/nb_cores)..((idx+1)*ni/nb_cores);
        writers.push((idx, out, ni_rng, cam.clone(), w.clone()));
    }

    // #############################################################################
    // #############################################################################
    // REMOVE FOR WINDOWS
    // --BEGIN-- vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    eprint!("\n\nRendering image...\n");
    eprint!(" |\x1b[48C|\x1b[1A\n");
    //for _ in 0..48 { eprint!("-"); }
    //eprint!("+\n |\x1b[48C|\n");
    //eprint!(" +");
    //for _ in 0..48 { eprint!("-"); }
    //eprint!("+\n\x1b[1A\n");
    //eprint!("\x1b[2A\n");
    // --END-- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // #############################################################################
    // #############################################################################


    writers.par_iter_mut().for_each(|(id, f, range, cam, w)| {

        // #########################################################################
        // #########################################################################
        // REMOVE FOR WINDOWS
        // --BEGIN-- vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        let color = {
            match *id {
                0 => "\x1b[31m",
                1 => "\x1b[32m",
                2 => "\x1b[33m",
                3 => "\x1b[34m",
                _ => "\x1b[30m",
            }
        };
        // --END-- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // #########################################################################
        // #########################################################################

        for i in range.rev() {

            // #####################################################################
            // #####################################################################
            // REMOVE FOR WINDOWS
            // --BEGIN-- vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            if i * 100 % ni == 0 {
                let load = 100 - i * 100 / ni;
                if *id % 2 == 1 {
                    if load % 2 == 0 || load % 25 == 0 {
                        eprint!("\x1b[2B\x1b[{}C{}█\x1b[2A", load/2 + *id/2, color);
                    } else {
                        eprint!("\x1b[2B\x1b[{}C{}▀\x1b[2A", (load+1)/2 + *id/2, color);
                    }
                } else {
                    if load % 2 == 0 && load % 25 != 0 {
                        eprint!("\x1b[2B\x1b[{}C{}▀\x1b[2A", load/2 + *id/2, color);
                    } else {
                        eprint!("\x1b[2B\x1b[{}C{}█\x1b[2A", (load-1)/2 + *id/2, color);
                    }
                }
                eprintln!("\x1b[0m");
            }
            // --END-- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            // ######################################################################
            // ######################################################################


            // ######################################################################
            // ######################################################################
            // REPLACE WITH
            // --BEGIN-- vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            // if i * 100 % ni == 0 {
            //     let load = 100 - i * 100 / ni;
            //     eprintln!("{}%", load);
            // }
            // --END-- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            // ######################################################################
            // ######################################################################

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
            writeln!(f, "").unwrap();
        }
    });
    print!("\n\n\n");
    let mut f = File::create("img.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", nj, ni).unwrap();
    for idx in (0..nb_cores).rev() {
        let output = Command::new("cat")
            .arg(&format!(".out{}.txt", idx))
            .output()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
        write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    }
}
