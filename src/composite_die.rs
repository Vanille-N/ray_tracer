use crate::hitable::*;
use crate::vec3::Vec3;
use crate::primitives::*;

#[derive(Clone, Copy)]

pub struct Die {
    pub a: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub side_texture: Texture,
    pub edge_texture: Texture,
    pub dot_texture: Texture,
}

impl Die {
    pub fn build(self) -> Composite {
        let len = self.w.len();
        let w = self.w.unit() * len; // Upwards
        let u = self.v.cross(&w).unit() * len;
        let v = w.cross(&u).unit() * len;

        let mut die: Interaction = Rhombus {
            a: self.a,
            u, v, w,
            texture: self.side_texture,
        }.orthonormal().build().wrap();
        die.intersect(Sphere {
            center: self.a + u/2. + v/2. + w/2.,
            radius: len * 0.75,
            texture: self.edge_texture,
        }.build());

        let mut make_dot = |x: f64, y: f64, z: f64| {
            die.remove(Sphere {
                center: self.a + u*(0.5 + x/2.) + v*(0.5 + y/2.) + w*(0.5 + z/2.),
                radius: len * 0.07,
                texture: self.dot_texture,
            }.build());
        };

        // 1
        make_dot(0.0, 0.0, 1.05);
        // 2
        make_dot(1.05, 0.4, 0.4);
        make_dot(1.05, -0.4, -0.4);
        // 3
        make_dot(0.4, 1.05, 0.4);
        make_dot(0.0, 1.05, 0.0);
        make_dot(-0.4, 1.05, -0.4);
        // 4
        make_dot(0.4, -1.05, 0.4);
        make_dot(-0.4, -1.05, -0.4);
        make_dot(0.4, -1.05, -0.4);
        make_dot(-0.4, -1.05, 0.4);
        // 5
        make_dot(-1.05, 0.4, 0.4);
        make_dot(-1.05, 0.4, -0.4);
        make_dot(-1.05, -0.4, 0.4);
        make_dot(-1.05, -0.4, -0.4);
        make_dot(-1.05, 0.0, 0.0);
        // 6
        make_dot(0.3, 0.4, -1.05);
        make_dot(0.3, 0.0, -1.05);
        make_dot(0.3, -0.4, -1.05);
        make_dot(-0.3, 0.4, -1.05);
        make_dot(-0.3, 0.0, -1.05);
        make_dot(-0.3, -0.4, -1.05);

        vec![die]
    }
}
