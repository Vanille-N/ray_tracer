# ray_tracer

## A Rust script for ray path tracing and image generation

Originally inspired by _Ray Tracing in One Weekend_ (Peter Shirley) 

### How to run this program

- Clone this repository
- If `rsmake` is not already executable then run `$ chmod u+x rsmake`
- `$ ./rsmake` will do a few things :
  - compile in release mode (debug mode is useless since rendering an image takes longer than compiling), make sure that `cargo` is in your `$PATH`
  - copy the executable to the root directory of the project as `exec`
  If `rsmake` fails, one common reason is the presence of multiple executables in `./release/deps/`. The problem can be fixed by deleting `./release/` before running `rsmake` again.
- Run `$ ./exec`
- Open the newly generated `img.ppm`

### Creating a new scene

To create and render a scene, edit the `build_world` function in `main.rs`, then run as described above. A list of objects and functions to do so can be found below.

### Creating a new complex object

It is recommended to create a new module :
- Create a new file `composite_<object>.rs`
- Implement `<object>` by providing a `build` method (more information below)
- Integrate the new object with the rest of the program

### Modules

```rust
mod camera;                       // Abstraction for the camera
mod hitable;                      // Logic for managing ray/object interaction
mod primitives;                   // Basic objects
mod ray;                          // Wrapper, no important logic in this file
mod rgb;                          // Color struct
mod vec3;                         // Vector overloads

mod composite_axes;               // Axes for debugging purposes
mod composite_cradle;             // Newton's craddle
mod composite_die;                // Cubic die
mod composite_molecules;          // Miscellaneous molecules
mod composite_erlenmeyer;         // Glass erlenmeyer
```

### Imports and dependencies
```rust
std::ops
std::fmt
std::fs
std::io
std::process

rand
rayon
```

## Contents

### vec3.rs
```rust
pub struct Vec3 {         // Derives Copy
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(f64, f64, f64) -> Self;                    // New vector from coordinates
    pub fn len(&self) -> f64;                             // Length
    pub fn unit(&self) -> Self;                           // Unit vector with same direction
    pub fn dot(&self, &Self) -> f64;                      // Dot product
    pub fn dot_self(&self) -> f64;                        // Dot product with self
    pub fn cross(&self, &Self) -> Self;                   // Cross product
    pub fn reflect(&self, &Self) -> Self;                 // Calculate reflection using the surface normal
    pub fn refract(&self, &Self, f64) -> Option<Self>;    // Calculate refraction using the surface normal and quotient of optical indexes
}

impl ops::Add for Vec3;
impl ops::AddAssign for Vec3;
impl ops::Mul<Vec3> for Vec3;                              // Each coordinate separately 
impl ops::MulAssign<Vec3> for Vec3;                        // Each coordinate separately
impl ops::Mul<f64> for Vec3;
impl ops::MulAssign<f64> for Vec3;
impl ops::Sub for Vec3;
impl ops::SubAssign for Vec3;
impl ops::Div<Vec3> for Vec3;                              // Each coordinate separately
impl ops::DivAssign<Vec3> for Vec3;                        // Each coordinate separately
impl ops::Div<f64> for Vec3;
impl ops::DivAssign<f64> for Vec3;
impl ops::Neg for Vec3;
```

### rgb.rs
```rust
pub struct RGB {    // derives Copy
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn new(f64, f64, f64) -> Self;                    // New color from r/g/b values in [0.; 1.]
}

impl ops::Add for RGB;
impl ops::AddAssign for RGB;
impl ops::Mul<RGB> for RGB ;                              // Each value separately
impl ops::MulAssign<RGB> for RGB;                         // Each value separately
impl ops::Mul<f64> for RGB;
impl ops::MulAssign<f64> for RGB;
impl ops::Sub for RGB;
impl ops::SubAssign for RGB;
impl ops::Div<RGB> for RGB;                               // Each value separately
impl ops::DivAssign<RGB> for RGB;                         // Each value separately
impl ops::Div<f64> for RGB;
impl ops::DivAssign<f64> for RGB;
impl fmt::Display for RGB;                                // For ppm output : "{r} {g} {b}"
impl ops::Rem<usize> for RGB;                             // COLOR%n == COLOR * n as f64 / 100.

pub const RED: RGB;
pub const DKRED: RGB;
pub const LTRED: RGB;
pub const BLUE: RGB;
pub const DKBLUE: RGB;
pub const LTBLUE: RGB;
pub const CYAN: RGB;
pub const GREEN: RGB;
pub const DKGREEN: RGB;
pub const LTGREEN: RGB;
pub const PURPLE: RGB;
pub const MAGENTA: RGB;
pub const YELLOW: RGB;
pub const BROWN: RGB;
pub const ORANGE: RGB;
pub const TURQUOISE: RGB;
pub const BLACK: RGB;
pub const WHITE: RGB;
pub const GREY: RGB;
pub const DKGREY: RGB;
pub const LTGREY: RGB;
```

### ray.rs
```rust
pub struct Ray {       // derives Copy
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(Vec3, Vec3) -> Self;            // Create ray from origin and direction
    pub fn project(&self, f64) -> Vec3;        // r.project(t) == r.orig + r.dir * t
}
```

### camera.rs
```rust
pub struct Camera {     // derives Clone
    orig: Vec3,
    low_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_absolute(
        Vec3,                // Look from
        Vec3,                // Look at
        Vec3,                // Vertical
        f64,                 // Field of view (degrees)
        f64,                 // Aspect ratio
    ) -> Self;

    pub fn new_relative(
        Vec3,                // Look at
        f64,                 // Angle around target (degrees)
        f64,                 // Angle above target (degrees)
        f64,                 // Distance from target
        f64,                 // Lateral tilt (degrees)
        f64,                 // Field of view (degrees)
        f64,                 // Aspect ratio
    ) -> Self;

    pub fn get_ray(&self, f64, f64) -> Ray;   // map [0.; 1.] x [0.; 1.] to rays going out of the camera
}
```

### primitives.rs
_Some types and traits are defined only later in `hitable.rs`_

```rust
pub struct Sphere {               // derives Copy, implements build as a method and Hit as a trait
    pub center: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct InfinitePlane {        // derives Copy, implements build as a method and Hit as a trait
    pub orig: Vec3,
    pub normal: Vec3,
    pub texture: Texture,
}

pub struct Triangle {             // derives Copy, implements build as a method and Hit as a trait
    pub a: Vec3,                  // One angle
    pub u: Vec3,                  // <──┬── Two sides of the triangle
    pub v: Vec3,                  // <──┘
    pub texture: Texture,
}

pub struct Parallelogram {        // derives Copy, implements build as a method and Hit as a trait
    pub a: Vec3,                  // One angle
    pub u: Vec3,                  // <──┬── Two sides of the parallelogram
    pub v: Vec3,                  // <──┘
    pub texture: Texture,
}

pub struct Rhombus {              // derives Copy, implements build as a method
    pub a: Vec3,                  // One angle
    pub u: Vec3,                  // <──┬── Three wedges of the Rhombus (actually a Parallelepiped, a.k.a. Rhomboid)
    pub v: Vec3,                  // <──┤
    pub w: Vec3,                  // <──┘
    pub texture: Texture,
}

pub struct RhombusObject(         // derives Copy, implements Hit as a trait
    pub [Parallelogram; 6]        // Six sides
);

impl Rhombus {
    pub fn orthogonal(self) -> Rhombus;    // Transform into a rectangular cuboid
    pub fn orthonormal(self) -> Rhombus;   // Transform into a cube
}

pub struct EmptyCylinder {       // derives Copy, implements build as a method and Hit as a trait
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct Disc {                // derives Copy, implements build as a method and Hit as a trait
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct Cylinder {            // derives Copy, implements build as a method
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct CylinderObject {      // derives Copy, implements Hit as a trait
    pub side: EmptyCylinder,
    pub cap1: Disc,
    pub cap2: Disc,
}

pub struct EmptyCone {           // derives Copy, implements build as a method and Hit as a trait
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,              // Angle in degrees
    pub begin: f64,              // Position of first cap
    pub end: f64,                // Position of second cap
    pub texture: Texture,
}
// begin and can take any value in ]-∞; +∞[, but should satisfy (begin < end)

pub struct Cone {               // derives Copy, implements build as a method
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
}

pub struct ConeObject {        // derives Copy, implements Hit as a trait
    pub side: EmptyCone,
    pub cap1: Disc,
    pub cap2: Disc,
}
```
The convention is that :
- if `XObject` exists, then `X` builds to an `XObject` wrapped in a `Primitive`. Various methods may be called on an instance of `X` before building. `X` does not implement `Hit`, only `XObject` does.
- in all other cases, the `build` method of `X` is just a wrapper to a `Primitive`.

### hitable.rs
```rust

pub enum Primitive {    // derives copy, implements Hit as a trait (wrapper)
    Sphere(Sphere),
    InfinitePlane(InfinitePlane),
    Triangle(Triangle),
    Parallelogram(Parallelogram),
    Rhombus(RhombusObject),
    EmptyCylinder(EmptyCylinder),
    Disc(Disc),
    Cylinder(CylinderObject),
    EmptyCone(EmptyCone),
    Cone(ConeObject),
}

impl Primitive {
    pub fn wrap(self) -> Interaction;
    pub fn intersect(self, Self) -> Interaction;
    pub fn remove(self, Self) -> Interaction; 
    pub fn texture(&self) -> Texture;                // Wrapper
}

pub struct Interaction(      // derives Clone
    Vec<Primitive>,          // Must be inside all of these...
    Vec<Primitive>,          // ...and outside all of these.
);

impl Interaction {
    pub fn bidir_hit(&Primitive, Vec3, Vec3) -> bool;   // A simple test for inside/outside: does a ray hit the object in both directions ?
    pub fn inside(obj: &Primitive, Vec3) -> bool;
    pub fn outside(obj: &Primitive, pos: Vec3) -> bool;
    pub fn intersect(&mut self, Primitive);            // Add other to the list of all inside
    pub fn remove(&mut self, Primitive);               // Add other to the list of all outside
    pub fn all_inside_except(Vec3, &[Primitive], usize) -> bool;      // Test that all objects in the interaction satisfy
    pub fn all_outside_except(Vec3, &[Primitive], usize) -> bool;     // the requirements
}

pub type Composite = Vec<Interaction>;       // A Composite is a collection of interactions between Primitives

pub struct ActiveHit {           // derives Copy
    pub t: f64,                  // Time of hit
    pub pos: Vec3,               // Position of hit
    pub normal: Vec3,            // Surface normal
    pub texture: Texture,        // Surface texture
}

impl ActiveHit {
    pub fn later(self, f64) -> Self;       // Apply translation to time
}

pub enum HitRecord {         // derives Copy
    Blank,
    Hit(ActiveHit),
}

impl HitRecord {
    pub fn make(f64, Vec3, Vec3, Texture) -> Self;       // Normalize the normal vector + wrap the rest
    pub fn compare(&mut self, Self);                     // The one with smallest t overwrites the other
}

pub trait Hit {
    fn hit(&self, &Ray) -> HitRecord;
}

pub struct World(             // derives Clone
    Composite,                // The World is just a particular extendable Composite
);

impl World {
    pub fn new() -> Self;
    pub fn push(&mut self, Interaction);       // Add to the world
    pub fn push_vec(&mut self, Composite);     // Unpack and add components individually
}

impl World {
    fn hit(&self, &Ray) -> HitRecord;                   // Wrapper around everything else
    pub fn caracteristics(&self, Vec3) -> (f64, RGB);   // Get optical index and color of a point in space. Only useful for refraction.
}

pub enum Texture {            // derives Copy
    Lambertian(RGB),          // Equivalent to Metal(*, 1.)
    Metal(RGB, f64),          // Reflective material with a color and a fuzziness
    Light(RGB),               // Does not reflect. Can be used with rgb components greater than 1. to emulate a light source
    Dielectric(RGB, f64),     // Reflects and refracts according to optical index
}

fn schlick(f64, f64, f64) -> f64;      // Schlick's approximation

pub fn scatter(&Ray, ActiveHit, &World) -> Option<(RGB, Ray)>;   // Calculate (with randomness) a refracted / reflected ray
pub fn color(&Ray, &World, i32, &Sky) -> RGB;                    // Recursively calculate color
fn random_in_unit_sphere() -> Vec3;                              // self-explanatory

pub struct Sky {                // derives Clone
    map: Vec<Vec<RGB>>,
    hgt: usize,
    wth: usize,
}

impl Sky {
    pub fn new(&str) -> Self;              // Provide a ppm file
    pub fn color(&self, Vec3) -> RGB;      // Project a direction on a background image
}
```

