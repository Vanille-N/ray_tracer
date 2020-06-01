use crate::internal::*;

/// A simple 6-sided die.
///
/// Its texture is highly customizable.
#[derive(Clone, Copy)]
pub struct Die {
    /// One of the angles
    pub a: Vec3,
    /// Direction of one edge adjacent to the reference angle
    pub up: Vec3,
    /// Rotation around said edge (in degrees)
    pub rot: f64,
    /// Texture of the area around dots
    pub side_texture: Texture,
    /// Texture of the carved out area at the corners
    pub edge_texture: Texture,
    /// Texture inside the dots
    pub dot_texture: Texture,
}

impl Die {
    pub fn build(self) -> Composite {
        let len = self.up.len();
        let w = self.up; // Upwards
        let u1 = {
            let u = w.cross(Vec3(0.0, 0.0, 1.0));
            if u.len() < EPSILON {
                w.cross(Vec3(1.0, 0.0, 0.0))
            } else {
                u
            }
            .unit()
                * len
        };
        let v1 = w.cross(u1).unit() * len;
        let v = u1 * self.rot.cos() + v1 * self.rot.sin();
        let u = -u1 * self.rot.sin() + v1 * self.rot.cos();

        let mut die: Interaction = Rhomboid {
            a: self.a,
            u,
            v,
            w,
            texture: self.side_texture,
        }
        .orthonormal()
        .build()
        .wrap()
        .intersect(
            Sphere {
                center: self.a + u / 2. + v / 2. + w / 2.,
                radius: len * 0.75,
                texture: self.edge_texture,
            }
            .build(),
        );

        let mut make_dot = |x: f64, y: f64, z: f64| {
            die.remove_mut(
                Sphere {
                    center: self.a + u * (0.5 + x / 2.) + v * (0.5 + y / 2.) + w * (0.5 + z / 2.),
                    radius: len * 0.07,
                    texture: self.dot_texture,
                }
                .build(),
            );
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
