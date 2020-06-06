use crate::internal::*;

/// This instance of a [Newton's cradle](https://en.wikipedia.org/wiki/Newton%27s_cradle)
/// can be placed anywhere in the space, but can only face upward.
#[derive(Clone, Copy)]
pub struct NewtonCradle {
    /// Position of one of the corners
    pub a: Vec3,
    /// Rotation around a vertical axis going through a (degrees)
    pub angle: f64,
    /// Scale (1.0 will have the size of a 1x1x1 cube)
    pub size: f64,
    /// Describes the positions of the balls
    pub pos: Option<[f64; 5]>
}

impl NewtonCradle {
    pub fn build(self) -> Composite {
        let plastic = Texture::Lambertian(RGB(0.1, 0.1, 0.1));
        let steel = Texture::Metal(RGB(0.8, 0.6, 0.2), 0.0);
        let nylon = Texture::Lambertian(RGB(0.9, 0.9, 0.9));
        let len = self.size;
        let angle = self.angle * std::f64::consts::PI / 180.;
        let w = Vec3(0.0, len, 0.0); // Upwards
        let u = Vec3(len * angle.cos(), 0.0, len * angle.sin());
        let v = Vec3(len * angle.sin(), 0.0, len * angle.cos());
        let pedestal = Rhomboid {
            a: self.a,
            u,
            v,
            w: w * 0.1,
            texture: plastic,
        }
        .build()
        .wrap();
        let make_pillar = |c1, c2| {
            EmptyCylinder {
                center1: c1,
                center2: c2,
                radius: v.len() * 0.03,
                texture: plastic,
            }
            .build()
            .wrap()
        };
        let pillar1 = make_pillar(self.a + v * 0.1 + u * 0.1, self.a + v * 0.1 + u * 0.1 + w);
        let pillar2 = make_pillar(self.a + v * 0.9 + u * 0.1, self.a + v * 0.9 + u * 0.1 + w);
        let pillar3 = make_pillar(self.a + v * 0.9 + u * 0.9, self.a + v * 0.9 + u * 0.9 + w);
        let pillar4 = make_pillar(self.a + v * 0.1 + u * 0.9, self.a + v * 0.1 + u * 0.9 + w);
        let bar1 = make_pillar(
            self.a + v * 0.1 + u * 0.1 + w,
            self.a + v * 0.9 + u * 0.1 + w,
        );
        let bar2 = make_pillar(
            self.a + v * 0.1 + u * 0.9 + w,
            self.a + v * 0.9 + u * 0.9 + w,
        );

        let make_cap = |c| {
            Sphere {
                center: c,
                radius: v.len() * 0.03,
                texture: plastic,
            }
            .build()
            .wrap()
        };
        let smoothtop1 = make_cap(self.a + v * 0.1 + u * 0.1 + w);
        let smoothtop2 = make_cap(self.a + v * 0.1 + u * 0.9 + w);
        let smoothtop3 = make_cap(self.a + v * 0.9 + u * 0.9 + w);
        let smoothtop4 = make_cap(self.a + v * 0.9 + u * 0.1 + w);
        let r = u.len() * 0.7 / 10.0;

        let make_ball = |c: Vec3, swing: f64| {
            let axis = Vec3(c.0, w.1, c.2);
            let theta = swing * std::f64::consts::PI / 180.;
            let radius = (c - axis).len() / w.len();
            let local_w = v * radius * theta.sin() + w * radius * theta.cos();
            let sphere = Sphere {
                center: axis - local_w ,
                radius: r,
                texture: steel,
            };
            let ring = Sphere {
                center: sphere.center + local_w.unit() * r,
                radius: r * 0.3,
                texture: plastic,
            };
            let threada = EmptyCylinder {
                center1: ring.center,
                center2: ring.center + u * 0.372 + local_w * 0.9,
                radius: r * 0.03,
                texture: nylon,
            }
            .build();
            let threadb = EmptyCylinder {
                center1: ring.center,
                center2: ring.center - u * 0.372 + local_w * 0.9,
                radius: r * 0.03,
                texture: nylon,
            }
            .build();
            (
                sphere.build().wrap(),
                ring.build().wrap(),
                threada.wrap(),
                threadb.wrap(),
            )
        };
        let pos = match self.pos {
            None => [0.0; 5],
            Some(arr) => arr,
        };
        let (sphere1, ring1, thread1a, thread1b) =
            make_ball(self.a + u * 0.5 + v * 0.15 + v.unit() * 9. * r + w * 0.3, pos[0]);
        let (sphere2, ring2, thread2a, thread2b) =
            make_ball(self.a + u * 0.5 + v * 0.15 + v.unit() * 7. * r + w * 0.3, pos[1]);
        let (sphere3, ring3, thread3a, thread3b) =
            make_ball(self.a + u * 0.5 + v * 0.15 + v.unit() * 5. * r + w * 0.3, pos[2]);
        let (sphere4, ring4, thread4a, thread4b) =
            make_ball(self.a + u * 0.5 + v * 0.15 + v.unit() * 3. * r + w * 0.3, pos[3]);
        let (sphere5, ring5, thread5a, thread5b) =
            make_ball(self.a + u * 0.5 + v * 0.15 + v.unit() * 1. * r + w * 0.3, pos[4]);
        vec![
            pedestal, pillar1, pillar2, pillar3, pillar4, bar1, bar2, smoothtop1, smoothtop2,
            smoothtop3, smoothtop4, sphere1, sphere2, sphere3, sphere4, sphere5, ring1, ring2,
            ring3, ring4, ring5, thread1a, thread1b, thread2a, thread2b, thread3a, thread3b,
            thread4a, thread4b, thread5a, thread5b,
        ]
    }
}
