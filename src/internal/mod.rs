pub mod camera;
pub mod hitable;
pub mod primitives;
pub mod ray;
pub mod rgb;
pub mod sky;
pub mod vec3;
pub mod world;

pub use hitable::*;
pub use primitives::*;
pub use rgb::RGB;
pub use world::World;
pub use vec3::Vec3;
pub use sky::Sky;
pub use ray::Ray;
pub use camera::Camera;

pub const EPSILON: f64 = 0.000_000_1;
