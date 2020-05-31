#![allow(dead_code)]

use libtrace::internal::*;
use libtrace::render::*;

fn main() {
    let builder = build_world();
    render(builder)
}

fn build_world() -> Builder {
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
    let sky = Sky::uniform(RGB(0.5, 0.5, 0.5));
    let mut world = World::new();

    world.set_background(RGB(0., 0., 0.));

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

    Builder {
        name: String::from("NeV"),
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
