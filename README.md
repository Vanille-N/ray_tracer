# ray_tracer

## A Rust script for ray path tracing and image generation

Originally inspired by _Ray Tracing in One Weekend_ (Peter Shirley).

### How to run this program

- Clone this repository
- Execute `cargo run --release` (not using `--release` is useless since execution is a lot longer than compilation)
- Open the newly generated `img.ppm`

### Creating a new scene

To create and render a scene, edit the `build_world` function in `main.rs`, then run as described above. A list of objects and functions to do so can be found in `DOC.md`.

### Creating a new complex object

It is recommended to create a new module :
- Create a new file `composite_<object>.rs`
- Implement `<object>` by providing a `build` method (more information below)
- Integrate the new object with the rest of the program

### What already existed ?

Of course, this project bears some resemblance with the original _Ray tracing in one Weekend_:
- The name of many types and functions are the same: `hirable::schlick`, `camera::Camera::get_ray`, `vec3::Vec3`, `ray::Ray`, `hitable::Hit`, `hitable::Texture::Lambertian`.
- Some ideas were kept: overloading `std::ops::Mul` for `RGB`, initial `camera::Camera` abstraction.
- Some chunks of code have been translated to Rust almost verbatim: `primitives::Sphere::hit`, `hitable::scatter`.

Many other similarities were partially or completely rewritten halfway into the project.

The `hit` implementations for many objects were inspired by or debugged with the help of a wide variety of websites: `hitable::{EmptyCylinder, EmptyCone, Triangle}` were implemented after comparing the implementations of intersection with a ray from at least a dozen sources, most of which had made widely different design choices and were hard to adapt.

`hitable::{InfinitePlane, Disc}` were done by myself without help, `hitable::Parallelogram` was adapted from `hitable::Triangle`, and all of the other are merely wrappers.

## What's new ?

- All composite objects (`composite_axes::Axes`, `composite_cradle::NewtonCradle`, `composite_die::Die`, `composite_erlenmeyer::Erlenmeyer`, `composite_molecules::Molecules`) are my own
- The intersection/removal mechanism implemented in `hitable::Interaction` was fully implemented without external inspiration
- Although the first versions of the `Texture::Dielectric` branch of `hitable::scatter` were copied, later versions were fully remade from scratch with a completely different approach, which (unlike the original one) correctly deals with dielectric/dielectric interfaces
- `hitable::HitRecord` was also fully revised
- The `hitable::Sky` texture was of my own initiative
- Brand new `camera::Camera` abstraction: instead of `(look_from, look_at, upwards, field_of_view_angle, aspect_ratio)`, it is much easier to fine-tune a view with `(look_at, angle_around_target, angle_above_target, distance_to_target, tilt, field_of_view_angle, aspect_ratio)`
- The original version did not support multithreading
- All scenes visible in `img/` are of my own creation
