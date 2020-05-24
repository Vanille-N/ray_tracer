pub mod camera;
pub mod hitable;
pub mod primitives;
pub mod ray;
pub mod rgb;
pub mod sky;
pub mod vec3;
pub mod world;

pub use camera::Camera;
pub use hitable::*;
pub use primitives::*;
pub use ray::Ray;
pub use rgb::RGB;
pub use sky::Sky;
pub use vec3::Vec3;
pub use world::World;

pub const EPSILON: f64 = 0.000_000_1;
