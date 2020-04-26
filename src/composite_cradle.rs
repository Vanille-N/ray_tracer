use crate::hitable::*;
use crate::primitives::*;
use crate::rgb::RGB;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct NewtonCradle {
    pub a: Vec3,
    pub angle: f64,
    pub size: f64,
}

impl NewtonCradle {
    pub fn build(self) -> Composite {
        let plastic = Texture::Lambertian(RGB::new(0.1, 0.1, 0.1));
        let steel = Texture::Metal(RGB::new(0.8, 0.6, 0.2), 0.0);
        let nylon = Texture::Lambertian(RGB::new(0.9, 0.9, 0.9));
        let len = self.size;
        let angle = self.angle  * std::f64::consts::PI / 180.;
        let w = Vec3::new(0.0, len, 0.0); // Upwards
        let u = Vec3::new(len * angle.cos(), 0.0, len * angle.sin());
        let v = Vec3::new(len * angle.sin(), 0.0, len * angle.cos());
        let pedestal = Rhombus {
            a: self.a,
            u,
            v,
            w: w * 0.1,
            texture: plastic,
        }
        .build();
        let pillar1 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.1,
            center2: self.a + v * 0.1 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let pillar2 = EmptyCylinder {
            center1: self.a + v * 0.9 + u * 0.1,
            center2: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let pillar3 = EmptyCylinder {
            center1: self.a + v * 0.9 + u * 0.9,
            center2: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let pillar4 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.9,
            center2: self.a + v * 0.1 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let bar1 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.1 + w,
            center2: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let bar2 = EmptyCylinder {
            center1: self.a + v * 0.1 + u * 0.9 + w,
            center2: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let smoothtop1 = Sphere {
            center: self.a + v * 0.1 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let smoothtop2 = Sphere {
            center: self.a + v * 0.1 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let smoothtop3 = Sphere {
            center: self.a + v * 0.9 + u * 0.9 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let smoothtop4 = Sphere {
            center: self.a + v * 0.9 + u * 0.1 + w,
            radius: v.len() * 0.03,
            texture: plastic,
        }
        .build();
        let r = u.len() * 0.7 / 10.0;
        let sphere1 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring1 = Sphere {
            center: sphere1.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread1a = EmptyCylinder {
            center1: ring1.center,
            center2: ring1.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let thread1b = EmptyCylinder {
            center1: ring1.center,
            center2: ring1.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let sphere2 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 3. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring2 = Sphere {
            center: sphere2.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread2a = EmptyCylinder {
            center1: ring2.center,
            center2: ring2.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let thread2b = EmptyCylinder {
            center1: ring2.center,
            center2: ring2.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let sphere3 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 5. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring3 = Sphere {
            center: sphere3.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread3a = EmptyCylinder {
            center1: ring3.center,
            center2: ring3.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let thread3b = EmptyCylinder {
            center1: ring3.center,
            center2: ring3.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let sphere4 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 7. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring4 = Sphere {
            center: sphere4.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread4a = EmptyCylinder {
            center1: ring4.center,
            center2: ring4.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let thread4b = EmptyCylinder {
            center1: ring4.center,
            center2: ring4.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let sphere5 = Sphere {
            center: self.a + u * 0.5 + v * 0.15 + v.unit() * 9. * r + w * 0.3,
            radius: r,
            texture: steel,
        };
        let ring5 = Sphere {
            center: sphere5.center + w.unit() * r,
            radius: r * 0.3,
            texture: plastic,
        };
        let thread5a = EmptyCylinder {
            center1: ring5.center,
            center2: ring5.center + u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        let thread5b = EmptyCylinder {
            center1: ring5.center,
            center2: ring5.center - u * 0.40 + w * 0.64,
            radius: r * 0.03,
            texture: nylon,
        }
        .build();
        vec![
            pedestal.wrap(),
            pillar1.wrap(),
            pillar2.wrap(),
            pillar3.wrap(),
            pillar4.wrap(),
            bar1.wrap(),
            bar2.wrap(),
            smoothtop1.wrap(),
            smoothtop2.wrap(),
            smoothtop3.wrap(),
            smoothtop4.wrap(),
            sphere1.build().wrap(),
            sphere2.build().wrap(),
            sphere3.build().wrap(),
            sphere4.build().wrap(),
            sphere5.build().wrap(),
            ring1.build().wrap(),
            ring2.build().wrap(),
            ring3.build().wrap(),
            ring4.build().wrap(),
            ring5.build().wrap(),
            thread1a.wrap(),
            thread1b.wrap(),
            thread2a.wrap(),
            thread2b.wrap(),
            thread3a.wrap(),
            thread3b.wrap(),
            thread4a.wrap(),
            thread4b.wrap(),
            thread5a.wrap(),
            thread5b.wrap(),
        ]
    }
}
