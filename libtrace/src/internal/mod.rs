/// Abstaction for the field of view
pub mod camera;
/// Wrappers, trait & textures
pub mod hitable;
/// Basic objects to build complex scenes
pub mod primitives;
/// Simple representation of a light ray
pub mod ray;
/// Operations on color values
pub mod rgb;
/// External environment texture
pub mod sky;
/// Used for both positions and 3D-vectors
pub mod vec3;
/// Main loop & wrapper struct
pub mod world;

pub use camera::Camera;
pub use hitable::*;
pub use primitives::*;
pub use ray::Ray;
pub use rgb::RGB;
pub use sky::Sky;
pub use vec3::Vec3;
pub use world::World;

/// To prevent "shadow acne"
pub const EPSILON: f64 = 0.000_000_1;
