#![allow(dead_code)]
#![allow(unused_imports)]

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

// Disable this if your terminal does not support ANSI color codes
const COLOR_TERM: bool = true;

fn main() {
    let cfg = build_world();
    let cfg = Arc::new(cfg);
    let nb_cores = 5;
    if COLOR_TERM {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    let pool = ThreadPool::new(nb_cores);
    let barrier = Arc::new(Barrier::new(nb_cores + 1));
    for id in 0..nb_cores {
        let mut stdout = BufWriter::new(File::create(&format!(".out{}.txt", id)).unwrap());
        let rng = (id * cfg.hgt / nb_cores)..((id + 1) * cfg.hgt / nb_cores);
        let barrier = barrier.clone();
        let cfg = cfg.clone();
        pool.execute(move || {
            let color = &format!("\x1b[3{}m", id + 1);
            let ni = cfg.hgt as f64;
            let nj = cfg.wth as f64;
            for i in rng.rev() {
                if COLOR_TERM {
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
                        c += world::calc_color(&r, &cfg.world, &cfg.sky);
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
    if COLOR_TERM {
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
        Command::new("rm")
            .arg(&format!(".out{}.txt", idx))
            .status()
            .expect("Failed to cleanup directory");
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
    let wth = 100; // width in pixels
    let hgt = 50; // height in pixels
    let iter = 20; // number of samples per pixel
    let cam = Camera::new_relative(
        Vec3(75.0, 15.0, 0.0),   // target
        -20.0,                   // angle (degrees)
        20.0,                    // rise (degrees)
        300.0,                   // distance (meters),
        0.0,                     // tilt (degrees)
        20.0,                    // aperture (degrees)
        wth as f64 / hgt as f64, // aspect ratio
    );
    let sky = Sky::blank();
    let mut world = World::new();

    world.dark_mode();

    let t1 = Texture::Lambertian(RGB(0.0, 0.7, 0.7));
    let t2 = Texture::Lambertian(RGB(0.2, 0.8, 0.3));
    let t3 = Texture::Lambertian(RGB(0.5, 0.7, 0.0));

    let x = Vec3(10., 0., 0.);
    let y = Vec3(0., 10., 0.);
    let z = Vec3(0., 0., 30.);

    let nbase = Rhomboid {
        a: -x * 0.2,
        u: y * 0.5,
        v: x * 1.4,
        w: z,
        texture: t1,
    }
    .build()
    .wrap();

    let nlbar = Rhomboid {
        a: x * 0.25,
        u: y * 4.55,
        v: x * 0.5,
        w: z,
        texture: t1,
    }
    .build()
    .wrap();

    let nlserif = Rhomboid {
        a: y * 0.5,
        u: y * 0.25,
        v: x,
        w: z,
        texture: t1,
    }
    .build()
    .remove(
        Cylinder {
            center1: y * 0.75 - z * 0.1,
            center2: y * 0.75 + z * 1.1,
            radius: 2.5,
            texture: t1,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x + y * 0.75 - z * 0.1,
            center2: x + y * 0.75 + z * 1.1,
            radius: 2.5,
            texture: t1,
        }
        .build(),
    );

    let nmidbar = Rhomboid {
        a: -x * 0.2 + y * 5.,
        u: x * 1.65,
        v: x * 5. - y * 5.,
        w: z,
        texture: t1,
    }
    .build()
    .remove(
        Rhomboid {
            a: x * 0.62 + y * 4.7 - z * 0.1,
            u: x * 0.6,
            v: x * 5. - y * 5.,
            w: z * 1.2,
            texture: t1,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x * 5.,
            center2: x * 7.,
            radius: 40.,
            texture: t1,
        }
        .build(),
    );

    let nrbar = Rhomboid {
        a: x * 4.8,
        u: y * 5.,
        v: x * 0.5,
        w: z,
        texture: t1,
    }
    .build()
    .wrap();

    let ntop = Rhomboid {
        a: x * 4.30 + y * 4.5,
        u: y * 0.5,
        v: x * 1.4,
        w: z,
        texture: t1,
    }
    .build()
    .wrap();

    let nrserif = Rhomboid {
        a: x * 4.55 + y * 4.25,
        u: y * 0.25,
        v: x,
        w: z,
        texture: t1,
    }
    .build()
    .remove(
        Cylinder {
            center1: x * 4.55 + y * 4.25 - z * 0.1,
            center2: x * 4.55 + y * 4.25 + z * 1.1,
            radius: 2.5,
            texture: t1,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x * 5.55 + y * 4.25 - z * 0.1,
            center2: x * 5.55 + y * 4.25 + z * 1.1,
            radius: 2.5,
            texture: t1,
        }
        .build(),
    );

    let ecirc = Cylinder {
        center1: x * 7.5 + y * 1.5,
        center2: x * 7.5 + y * 1.5 + z,
        radius: 15.,
        texture: t2,
    }
    .build()
    .remove(
        Cylinder {
            center1: x * 7.5 + y * 1.5 - z * 0.1,
            center2: x * 7.5 + y * 1.5 + z * 1.1,
            radius: 10.,
            texture: t2,
        }
        .build(),
    )
    .remove(
        Rhomboid {
            a: x * 7.5 + y * 1.5 - z * 0.1,
            u: x * 10. + y * 5.,
            v: x * 10. - y * 5.,
            w: z * 1.2,
            texture: t2,
        }
        .build(),
    );

    let ehbar = Rhomboid {
        a: x * 6.5 + y * 1.,
        u: x * 2. + y * 1.,
        v: y * 0.57,
        w: z,
        texture: t2,
    }
    .build()
    .wrap();

    let vlbar = Rhomboid {
        a: x * 9.5,
        u: y * 5.,
        v: x * 0.5,
        w: z,
        texture: t3,
    }
    .build()
    .wrap();

    let vltop = Rhomboid {
        a: x * 9. + y * 4.5,
        u: y * 0.5,
        v: x * 1.4,
        w: z,
        texture: t3,
    }
    .build()
    .wrap();

    let vlserif = Rhomboid {
        a: x * 9.25 + y * 4.25,
        u: y * 0.25,
        v: x,
        w: z,
        texture: t3,
    }
    .build()
    .remove(
        Cylinder {
            center1: x * 9.25 + y * 4.25 - z * 0.1,
            center2: x * 9.25 + y * 4.25 + z * 1.1,
            radius: 2.5,
            texture: t3,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x * 10.25 + y * 4.25 - z * 0.1,
            center2: x * 10.25 + y * 4.25 + z * 1.1,
            radius: 2.5,
            texture: t3,
        }
        .build(),
    );

    let vmidlo = Cylinder {
        center1: x * 5.13 + y * 9.88,
        center2: x * 5.13 + y * 9.88 + z,
        radius: 110.3,
        texture: t3,
    }
    .build()
    .intersect(
        Rhomboid {
            a: x * 9.5,
            u: y * 5.,
            v: x * 6.,
            w: z * 1.2,
            texture: t3,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x * 5.13 + y * 9.88 - z * 0.1,
            center2: x * 5.13 + y * 9.88 + z * 1.1,
            radius: 105.,
            texture: t3,
        }
        .build(),
    );

    let vmidhi = Cylinder {
        center1: x * 5.13 + y * 9.88,
        center2: x * 5.13 + y * 9.88 + z,
        radius: 100.,
        texture: t3,
    }
    .build()
    .intersect(
        Rhomboid {
            a: x * 9.5,
            u: y * 5.,
            v: x * 6.,
            w: z * 1.2,
            texture: t3,
        }
        .build(),
    )
    .remove(
        Cylinder {
            center1: x * 5.13 + y * 9.88 - z * 0.1,
            center2: x * 5.13 + y * 9.88 + z * 1.1,
            radius: 95.,
            texture: t3,
        }
        .build(),
    );

    let vrtop = Rhomboid {
        a: x * 13.5 + y * 4.5,
        u: y * 0.5,
        v: x * 0.9,
        w: z,
        texture: t3,
    }
    .build()
    .wrap();

    world.push(nbase);
    world.push(nlbar);
    world.push(nlserif);
    world.push(nmidbar);
    world.push(nrbar);
    world.push(ntop);
    world.push(nrserif);
    world.push(ecirc);
    world.push(ehbar);
    world.push(vlbar);
    world.push(vlserif);
    world.push(vltop);
    world.push(vmidlo);
    world.push(vmidhi);
    world.push(vrtop);

    Cfg {
        hgt,
        wth,
        iter,
        cam,
        world,
        sky,
    }
}
