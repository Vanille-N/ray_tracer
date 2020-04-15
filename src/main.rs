extern crate rand;
extern crate rayon;
//extern crate cmd_lib;
use rand::Rng;
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
mod composite_craddle;
mod composite_molecules;

use vec3::Vec3;
use rgb::RGB;
use hitable::*;
use camera::Camera;
use primitives::*;
use composite_craddle::*;
use composite_molecules::*;


fn main() {
    let mut rng = rand::thread_rng();
    let nj = 200; // width in pixels
    let ni = 200; // height in pixels
    let ns = 50; // number of samples per pixel
    let mut out1 = File::create(".out1.txt").unwrap();
    let mut out2 = File::create(".out2.txt").unwrap();
    let mut out3 = File::create(".out3.txt").unwrap();
    let mut out4 = File::create(".out4.txt").unwrap();
    let mut file = File::create("img.ppm").unwrap();
    let ni1 = 0..ni/4;
    let ni2 = ni/4..ni/2;
    let ni3 = ni/2..3*ni/4;
    let ni4 = 3*ni/4..ni;
    // let cam = Camera::new_absolute(
    //     Vec3::new(-2.0, 3.0, -6.0), // eye
    //     Vec3::new(0.0, 0.5, 0.0), // target
    //     Vec3::new(0.0, 1.0, 0.0), // up
    //     40.0, // aperture (degrees)
    //     nj as f64 / ni as f64, // aspect ratio
    // );
    let cam = Camera::new_relative(
        Vec3::new(-10.0, 0.0, 0.0), // target
        17.0, // angle (degrees)
        45.0, // rise (degrees)
        55.0, // distance (meters),
        0.0, // tilt (degrees)
        50.0, // aperture (degrees)
        nj as f64 / ni as f64, // aspect ratio
    );
    let mut w = World::new();
    // let center = Sphere {
    //     center: Vec3::new(-0.0, 0.0, -1.0),
    //     radius: 0.5,
    //     texture: Texture::Lambertian(RGB::new(0.8, 0.3, 0.3)),
    // };
    let ground = InfinitePlane {
        orig: Vec3::new(0.0, -10.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        texture: Texture::Metal(RGB::new(0.2, 0.2, 0.1), 0.1),
    }.build().wrap();
    // let left = Sphere {
    //     center: Vec3::new(1.0, 0.0, -1.0),
    //     radius: 0.5,
    //     texture: Texture::Metal(RGB::new(0.8, 0.6, 0.2), 0.0),
    // };
    // let right = Sphere {
    //     center: Vec3::new(-1.0, 0.0, -1.0),
    //     radius: 0.5,
    //     texture: Texture::Lambertian(RGB::new(0.3, 0.5, 0.6)),
    // };
    let sun = Sphere {
        center: Vec3::new(-100.0, 100.0, -50.0),
        radius: 50.0,
        texture: Texture::Light(RGB::new(3.0, 3.0, 3.0)),
    }.build().wrap();
    // let cube = Rhombus {
    //     a: Vec3::new(2.0, 0.0, 1.0),
    //     u: Vec3::new(0.0, 0.0, 1.0),
    //     v: Vec3::new(0.0, 1.0, 0.0),
    //     w: Vec3::new(1.0, 0.0, 0.0),
    //     texture: Texture::Metal(RGB::new(0.6, 0.2, 0.7), 0.1),
    // }.build();
    let craddle = NewtonCraddle {
        a: Vec3::new(-20.0, -10.0, -20.0),
        u: Vec3::new(20.0 * 0.5, 0.0, 20.0 * 0.87),
        v: Vec3::new(20.0 * 0.87, 0.0, -20.0 * 0.5),
        w: Vec3::new(0.0, 20.0, 0.0),
    }.build().wrap();
    // let cyl = Cylinder {
    //     center1: Vec3::new(0.0, 0.5, 0.0),
    //     center2: Vec3::new(0.0, 0.5, 1.0),
    //     radius: 0.01,
    //     texture: Texture::Metal(RGB::new(0.2, 0.2, 0.2), 0.0),
    // }.build();
    let cyc = Molecule {
        c_ref: Vec3::new(-5.0, 0.7, 17.0),
        up: Vec3::new(0.3, 0.3, 0.0),
        fwd: Vec3::new(-1.0, 0.5, 1.0),
    }.cyclohexanol().build().wrap();
    let water = Molecule {
        c_ref: Vec3::new(-10.0, 10.0, 30.0),
        up: Vec3::new(0.1, 0.3, 0.0),
        fwd: Vec3::new(-1.0, 0.5, 1.0),
    }.water().build().wrap();
    let methane = Molecule {
        c_ref: Vec3::new(6.0, -5.0, 0.0),
        up: Vec3::new(0.1, 0.3, 0.0),
        fwd: Vec3::new(-1.0, 0.5, 1.0),
    }.methane().build().wrap();
    let ethanol = Molecule {
        c_ref: Vec3::new(-10.0, 1.0, -43.0),
        up: Vec3::new(0.1, 0.3, 0.0),
        fwd: Vec3::new(-1.0, 0.5, 1.0),
    }.ethanol().build().wrap();
    //w.push(cube);
    //w.push(center);
    w.push(ground);
    w.push(cyc);
    w.push(water);
    w.push(craddle);
    w.push(methane);
    w.push(ethanol);
    // //w.push(left);
    // //w.push(right);
    w.push(sun);
    let mut writers = [
        (3, out4, ni4, cam.clone(), w.clone()),
        (2, out3, ni3, cam.clone(), w.clone()),
        (1, out2, ni2, cam.clone(), w.clone()),
        (0, out1, ni1, cam.clone(), w.clone())
    ];


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
    let output = Command::new("cat").arg(".out4.txt").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    let output = Command::new("cat").arg(".out3.txt").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    let output = Command::new("cat").arg(".out2.txt").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
    let output = Command::new("cat").arg(".out1.txt").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}
