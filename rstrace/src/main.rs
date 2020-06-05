#![allow(dead_code)]

use pytrace_core as libtrace;

use libtrace::internal::*;
use libtrace::composite::*;
use libtrace::render::*;

fn main() {
    let builder = build_world();
    render(builder)
}

fn build_world() -> Builder {
    let wth = 1000; // width in pixels
    let hgt = 1000; // height in pixels
    let iter = 100; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3(0.0, 0.5, 0.0),   // target
        90.0,                   // angle (degrees)
        40.0,                    // rise (degrees)
        2.0,                   // distance (meters),
        0.0,                     // tilt (degrees)
        90.0,                    // aperture (degrees)
        wth as f64 / hgt as f64, // aspect ratio
    );
    let sky = Sky::uniform(RGB(0.5, 0.5, 0.5));
    let mut world = World::new();

    world.set_background(RGB(0., 0., 0.));

    let cradle = NewtonCradle {
        a: Vec3(-0.5, 0.0, -0.5),
        size: 1.,
        angle: 0.,
        pos: Some([0., 0., 90., 0., 0.]),
    }.build();

    world.push_vec(cradle);

    Builder {
        name: String::from("newt"),
        silent: false,
        hgt,
        wth,
        iter,
        cam,
        world,
        sky,
        nbsync: 5,
    }
}
