use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;
use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;

use crate::internal::*;

pub struct Builder {
    pub name: String,
    pub silent: bool,
    pub hgt: usize,
    pub wth: usize,
    pub iter: usize,
    pub cam: Camera,
    pub world: World,
    pub sky: Sky,
}

pub fn render(build: Builder) {
    let build = Arc::new(build);
    let nb_cores = 5;
    if !build.silent {
        eprint!("\n\nRendering image...\n");
        eprint!("|\x1b[50C|\x1b[1A\n");
    }
    let pool = ThreadPool::new(nb_cores);
    let barrier = Arc::new(Barrier::new(nb_cores + 1));
    for id in 0..nb_cores {
        let mut stdout =
            BufWriter::new(File::create(&format!(".out-{}-{}.txt", &build.name, id)).unwrap());
        let rng = (id * build.hgt / nb_cores)..((id + 1) * build.hgt / nb_cores);
        let barrier = barrier.clone();
        let build = build.clone();
        pool.execute(move || {
            let color = &format!("\x1b[3{}m", id + 1);
            let ni = build.hgt as f64;
            let nj = build.wth as f64;
            for i in rng.rev() {
                if !build.silent && i * 100 % build.hgt == 0 {
                    let load = 100 - i * 100 / build.hgt;
                    if load % 2 == 0 {
                        eprint!("\x1b[{}C{}█\x1b[1A\n", load / 2, color);
                    }
                }

                for j in 0..build.wth {
                    let mut c = rgb::BLACK;
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
    let mut f = File::create(&format!("img-{}.ppm", &build.name)).unwrap();
    writeln!(f, "P3\n{} {}\n255", build.wth, build.hgt).unwrap();
    for idx in (0..nb_cores).rev() {
        let output = Command::new("cat")
            .arg(&format!(".out-{}-{}.txt", &build.name, idx))
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        write!(f, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
        Command::new("rm")
            .arg(&format!(".out-{}-{}.txt", &build.name, idx))
            .status()
            .expect("Failed to cleanup directory");
    }
}
