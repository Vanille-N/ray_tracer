# PyTrace (core)

Originally inspired by _Ray Tracing in One Weekend_ (Peter Shirley).

This library cannot be executed as-is, but it can provide tools that both `rstrace` and `pytrace` can then use.

What can be done here is creating a new complex object.

To do so, it is recommended to create a new file in the `composite` module :
- Create a new file `composite/<object>.rs`
- Implement `<object>` by providing a `build` method (more information below)
- Integrate the new object with the rest of the program by adding the two lines `pub mod <object>;` and `pub use <object>::<Object>;` in `composite/mod.rs` (see examples)

### What already existed ?

Of course, this project bears some resemblance with the original _Ray tracing in one Weekend_:
- The name of many types and functions are the same: `internal::hitable::schlick`, `internal::camera::Camera::get_ray`, `internal::vec3::Vec3`, `internal::ray::Ray`, `internal::hitable::Hit`, `internal::hitable::Texture::Lambertian`.
- Some ideas were kept: overloading `std::ops::Mul` for `internal::rgb::RGB`, initial `internal::camera::Camera` abstraction.
- Some chunks of code have been translated to Rust almost verbatim: `internal::primitives::Sphere::hit`, `internal::hitable::scatter`.

Many other similarities were partially or completely rewritten halfway into the project.

The `hit` implementations for many objects were inspired by or debugged with the help of a wide variety of websites: `internal::hitable::{EmptyCylinder, EmptyCone, Triangle}` were implemented after comparing the implementations of intersection with a ray from at least a dozen sources, most of which had made widely different design choices and were hard to adapt.

`internal::hitable::{InfinitePlane, Disc}` were done by myself without help, `internal::hitable::Parallelogram` was adapted from `internal::hitable::Triangle`, and all of the other are merely wrappers.

## What's new ?

- All composite objects (`composite::axes::Axes`, `composite::cradle::NewtonCradle`, `composite::die::Die`, `composite::erlenmeyer::Erlenmeyer`, `composite::molecules::Molecules`) are my own
- The intersection/removal mechanism implemented in `internal::hitable::Interaction` was fully implemented without external inspiration
- Although the first versions of the `internal::hitable::Texture::Dielectric` branch of `internal::hitable::scatter` were copied, later versions were fully remade from scratch with a completely different approach, which (unlike the original one) correctly deals with dielectric/dielectric interfaces
- `internal::hitable::HitRecord` was also fully revised
- The `internal::sky::Sky` texture was of my own initiative
- Brand new `internal::camera::Camera` abstraction: instead of `(look_from, look_at, upwards, field_of_view_angle, aspect_ratio)`, it is much easier to fine-tune a view with `(look_at, angle_around_target, angle_above_target, distance_to_target, tilt, field_of_view_angle, aspect_ratio)`
- The original version did not support multithreading
- All scenes visible in `../img/` are of my own creation
