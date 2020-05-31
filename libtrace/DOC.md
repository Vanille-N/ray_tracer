# Overview of the types and functions

```diff
- WARNING
This document may not be perfectly up to date.
```
```diff
+ LAST UPDATED: 2020-05-17
(up to date with the code commited under 3c53f9e)
```

```diff
@@ DISCLAIMER @@
This document will not be updated anymore, the official documentation
(`$ cargo doc --open`) is being created and will (hopefully) soon catch up to
and overtake the document below.
```

### Modules

```rust
mod camera;                       // Abstraction for the camera
mod hitable;                      // Logic for managing ray/object interaction
mod primitives;                   // Basic objects
mod ray;                          // Wrapper, no important logic in this file
mod rgb;                          // Color struct
mod vec3;                         // Vector overloads
mod world;                        // Integration of all objects with main loop
mod sky;                          // Environment texture

mod composite_axes;               // Axes for debugging purposes
mod composite_cradle;             // Newton's craddle
mod composite_die;                // Cubic die
mod composite_molecules;          // Miscellaneous molecules
mod composite_flasks;             // Glass objects
```

### Imports and dependencies
```rust
std::ops
std::fmt
std::fs
std::io
std::process
std::sync

rand
threadpool
```

## Contents

### vec3.rs
```rust
pub struct Vec3(pub f64, pub f64, pub f64);  // derives Copy

impl Vec3 {
    pub fn len(&self) -> f64;                             // Length
    pub fn unit(&self) -> Self;                           // Unit vector with same direction
    pub fn dot(&self, &Self) -> f64;                      // Dot product
    pub fn dot_self(&self) -> f64;                        // Dot product with self
    pub fn cross(&self, Self) -> Self;                    // Cross product
    pub fn reflect(&self, Self) -> Self;                  // Calculate reflection using the surface normal
    pub fn refract(&self, Self, f64) -> Option<Self>;     // Calculate refraction using the surface normal and quotient of optical indexes
    pub fn random_unit() -> Self;                         // vector in unit sphere
}

impl ops::Add for Vec3;
impl ops::AddAssign for Vec3;
impl ops::Mul<Vec3> for Vec3;                             // Each coordinate separately
impl ops::MulAssign<Vec3> for Vec3;                       // Each coordinate separately
impl ops::Mul<f64> for Vec3;
impl ops::MulAssign<f64> for Vec3;
impl ops::Sub for Vec3;
impl ops::SubAssign for Vec3;
impl ops::Div<Vec3> for Vec3;                             // Each coordinate separately
impl ops::DivAssign<Vec3> for Vec3;                       // Each coordinate separately
impl ops::Div<f64> for Vec3;
impl ops::DivAssign<f64> for Vec3;
impl ops::Neg for Vec3;
```

### rgb.rs
```rust
pub struct RGB(pub f64, pub f64, pub f64)   // derives Copy

impl ops::Add for RGB;
impl ops::AddAssign for RGB;
impl ops::Mul<RGB> for RGB ;                // Each value separately
impl ops::MulAssign<RGB> for RGB;           // Each value separately
impl ops::Mul<f64> for RGB;
impl ops::MulAssign<f64> for RGB;
impl ops::Sub for RGB;
impl ops::SubAssign for RGB;
impl ops::Div<RGB> for RGB;                 // Each value separately
impl ops::DivAssign<RGB> for RGB;           // Each value separately
impl ops::Div<f64> for RGB;
impl ops::DivAssign<f64> for RGB;
impl fmt::Display for RGB;                  // For ppm output : "{r} {g} {b}"
impl ops::Rem<usize> for RGB;               // COLOR%n == COLOR * n as f64 / 100.
```

Consts are also available for most colors.

### ray.rs
```rust
pub struct Ray {                            // derives Copy
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(Vec3, Vec3) -> Self;        // Create ray from origin and direction
    pub fn project(&self, f64) -> Vec3;    // r.project(t) == r.orig + r.dir * t
}
```

### camera.rs
```rust
pub struct Camera {                           // derives Clone
    orig: Vec3,
    low_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_absolute(
        Vec3,                                 // Look from
        Vec3,                                 // Look at
        Vec3,                                 // Vertical
        f64,                                  // Field of view (degrees)
        f64,                                  // Aspect ratio
    ) -> Self;

    pub fn new_relative(
        Vec3,                                 // Look at
        f64,                                  // Angle around target (degrees)
        f64,                                  // Angle above target (degrees)
        f64,                                  // Distance from target
        f64,                                  // Lateral tilt (degrees)
        f64,                                  // Field of view (degrees)
        f64,                                  // Aspect ratio
    ) -> Self;

    pub fn get_ray(&self, f64, f64) -> Ray;   // map [0.; 1.] x [0.; 1.] to rays going out of the camera
}
```

### primitives.rs
```rust
pub struct Sphere {                     // derives Copy, implements build as a method and Hit as a trait
    pub center: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct InfinitePlane {              // derives Copy, implements build as a method and Hit as a trait
    pub orig: Vec3,
    pub normal: Vec3,
    pub texture: Texture,
}

pub struct Triangle {                   // derives Copy, implements build as a method and Hit as a trait
    pub a: Vec3,                        // One angle
    pub u: Vec3,                        // <──┬── Two sides of the triangle
    pub v: Vec3,                        // <──┘
    pub texture: Texture,
}

pub struct Parallelogram {              // derives Copy, implements build as a method and Hit as a trait
    pub a: Vec3,                        // One angle
    pub u: Vec3,                        // <──┬── Two sides of the parallelogram
    pub v: Vec3,                        // <──┘
    pub texture: Texture,
}

pub struct Rhomboid {                   // derives Copy, implements build as a method
    pub a: Vec3,                        // One angle
    pub u: Vec3,                        // <──┬── Three wedges of the Rhomboid (in the sense of 'parallelepiped')
    pub v: Vec3,                        // <──┤
    pub w: Vec3,                        // <──┘
    pub texture: Texture,
}

pub struct RhomboidObject(              // derives Copy, implements Hit as a trait
    pub [Parallelogram; 6]              // Six sides
);

impl Rhomboid {
    pub fn orthogonal(self) -> Self;    // Transform into a rectangular cuboid
    pub fn orthonormal(self) -> Self;   // Transform into a cube
}

pub struct EmptyCylinder {              // derives Copy, implements build as a method and Hit as a trait
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct Disc {                       // derives Copy, implements build as a method and Hit as a trait
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct Cylinder {                   // derives Copy, implements build as a method
    pub center1: Vec3,
    pub center2: Vec3,
    pub radius: f64,
    pub texture: Texture,
}

pub struct CylinderObject {             // derives Copy, implements Hit as a trait
    pub side: EmptyCylinder,
    pub cap1: Disc,
    pub cap2: Disc,
}

pub struct EmptyCone {                  // derives Copy, implements build as a method and Hit as a trait
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,                     // Angle in degrees
    pub begin: f64,                     // Position of first cap
    pub end: f64,                       // Position of second cap
    pub texture: Texture,
}
// `begin` and `end` can take any value in ]-∞ ; +∞ [, but should satisfy `begin < end`

pub struct Cone {                      // derives Copy, implements build as a method
    pub orig: Vec3,
    pub dir: Vec3,
    pub angle: f64,
    pub begin: f64,
    pub end: f64,
    pub texture: Texture,
}

pub struct ConeObject {               // derives Copy, implements Hit as a trait
    pub side: EmptyCone,
    pub cap1: Disc,
    pub cap2: Disc,
}
```
The convention is that :
- if `XObject` exists, then `X` builds to an `XObject` wrapped in a `Primitive`. Usually, this is because some amount of calculation has to be done once when the object is built instead of being done at each iteration of the loop. Various methods may be called on an instance of `X` before building. `X` does not implement `Hit`, only `XObject` does.
- in all other cases, the `build` method of `X` is just a wrapper to a `Primitive`.

### hitable.rs
```rust
// Objects need to be shared between threads
pub trait Hit: Send + Sync {
    fn hit(&self, &Ray) -> HitRecord;
    fn texture(&self) -> Texture;
    fn inside(&self, Vec3) -> bool;
}

pub struct ActiveHit {                                                // derives Copy
    pub t: f64,                                                       // Time of hit
    pub pos: Vec3,                                                    // Position of hit
    pub normal: Vec3,                                                 // Surface normal
    pub texture: Texture,                                             // Surface texture
}

impl ActiveHit {
    pub fn later(self, f64) -> Self;                                  // Apply translation to time
}

pub enum HitRecord {                                                  // derives Copy
    Blank,
    Hit(ActiveHit),
}

impl HitRecord {
    pub fn make(f64, Vec3, Vec3, Texture) -> Self;                    // Normalize the normal vector + wrap the rest
    pub fn compare(&mut self, Self);                                  // The one with smallest t overwrites the other
}

pub struct Primitive(pub Box<dyn Hit>);                               // The fact that a `Primitive` does not implement `Clone` is a guarantee that no excessive duplication of objects is done

impl Primitive {
    pub fn wrap(self) -> Interaction;
    pub fn intersect(self, Self) -> Interaction;
    pub fn remove(self, Self) -> Interaction;
    pub fn texture(&self) -> Texture;                                 // Wrapper
    pub fn hit(&self, &Ray) -> HitRecord                              // Wrapper
    pub fn inside(&self, Vec3) -> bool                                // Wrapper
}

pub struct Interaction(                                               // A point 'inside' an interaction...
    pub Vec<Primitive>,                                               // ...must be inside all of these...
    pub Vec<Primitive>,                                               // ...and outside all of these.
);

impl Interaction {
    pub fn bidir_hit<T: Hit>(&T, Vec3, Vec3) -> bool;                 // A simple test for inside/outside: does a ray hit the object in both directions ?
    pub fn inside(obj: &Primitive, Vec3) -> bool;
    pub fn outside(obj: &Primitive, pos: Vec3) -> bool;
    pub fn intersect(&mut self, Primitive);                           // Add other to the list of all inside
    pub fn remove(&mut self, Primitive);                              // Add other to the list of all outside
    pub fn all_inside_except(Vec3, &[Primitive], usize) -> bool;      // Test that all objects in the interaction satisfy the requirements
    pub fn all_outside_except(Vec3, &[Primitive], usize) -> bool;
}

pub type Composite = Vec<Interaction>;                                // A Composite is a collection of interactions between Primitives

pub enum Texture {                                                    // derives Copy
    Lambertian(RGB),                                                  // Equivalent to Metal(*, 1.)
    Metal(RGB, f64),                                                  // Reflective material with a color and a fuzziness
    Light(RGB),                                                       // Does not reflect. Can be used with rgb components greater than 1. to emulate a light source
    Dielectric(RGB, f64),                                             // Reflects and refracts according to optical index
}
```
### world.rs
```rust

pub struct World(                                                // Not clonable
    Composite,                                                   // The World is just a particular extendable Composite
);

impl World {
    pub fn new() -> Self;
    pub fn push(&mut self, Interaction);                         // Add to the world
    pub fn push_vec(&mut self, Composite);                       // Unpack and add components individually
    fn hit(&self, &Ray) -> HitRecord;                            // Wrapper around everything else
    pub fn caracteristics(&self, Vec3) -> (f64, RGB);            // Get optical index and color of a point in space. Only useful for refraction.
}


fn schlick(f64, f64, f64) -> f64;                                // Schlick's approximation
pub fn scatter(&Ray, ActiveHit, &World) -> Option<(RGB, Ray)>;   // Calculate (with randomness) a refracted / reflected ray
pub fn color(&Ray, &World, i32, &Sky) -> RGB;                    // Recursively calculate color
```

### sky.rs
```rust
pub struct Sky {                           // derives Clone
    map: Vec<Vec<RGB>>,
    hgt: usize,
    wth: usize,
}

impl Sky {
    pub fn new(&str) -> Self;              // Provide a ppm file (name)
    pub fn color(&self, Vec3) -> RGB;      // Project a direction on a background image
}
```


## Composite objects

The following are spread over the files:
- `composite_die.rs`
- `composite_axes.rs`
- `composite_cradle.rs`
- `composite_molecules.rs`
- `composite_flasks.rs`

All objects implement a `build` method to create a `Composite` by consuming themselves.

Additionally, they all derive `Copy`, but it should be noted that since `Composite` does not, they can only be copied before being built.

Generally speaking, it is recommended not to modify a `Composite` once it has been built: a single object may be split in several parts, and mutating a `Composite` will almost certainly wreck all composite objects (though dice will be fine).

### `composite_axes.rs`
```rust
pub struct Axes(pub f64);
```

`Axes` (for debugging purposes mostly) take a single float and interprete it as a length. They serve as both a scale and a help for orienting the view. Although adding `Axes` to a scene may slow the rendering, it can help positioning objects faster.

### `composite_die.rs`
```rust
pub struct Die {
    pub a: Vec3,
    pub up: Vec3,
    pub rot: f64,
    pub side_texture: Texture,
    pub edge_texture: Texture,
    pub dot_texture: Texture,
}
```

A standard cubic die, the textures of sides, dots, and edges can be controlled separately.
This object is composed of a single `Interaction`, it `build`s to a `Composite` merely because all other composite objects do so.

### `composite_cradle.rs`
```rust
pub struct NewtonCradle {
    pub a: Vec3,
    pub angle: f64,
    pub size: f64,
}
```

The cradle can only be facing upwards, it is composed of 5 balls, a stand, and 10 threads.

### `composite_flasks.rs`

```rust
pub struct Flask {
    pub a: Vec3,
    pub size: f64,
    pub color: RGB,
}

impl Flask {
    pub fn erlenmeyer(self) -> Composite;
    pub fn florence(self) -> Composite;    // Work in progress
}
```

Similarly, all flasks can only face upwards. `a` is the center of the bottom part, `color` is the shade of the solution.

Flasks are made of a very transparent glass with optical index 1.3, and filled with a colored solution. Being made of multiple intersections and removals of dielectrics and shapes such as cones and cylinders, flasks, and in particular erlenmeyers can be considered the most complex objects in this whole project as of now.

### `composite_molecules.rs`
```rust
pub struct Molecule {         // derives Copy
    pub c_ref: Vec3,
    pub up: Vec3,
    pub fwd: Vec3,
}

pub struct MoleculeObject {   // derives Clone
    pub atoms: Vec<Sphere>,
    pub links: Vec<EmptyCylinder>,
}
```

Molecules, like rhomboids, are built in two steps. The user provides :
- `c_ref` the center of one atom arbitrarily chosen as reference
- `up` a direction for the first link
- `fwd` to disambiguate orientation

There are also functions whose role is to create closures that make the creation of new atoms and links easier.

```rust
fn atom_builder(f64, Texture) -> Box<dyn Fn(Vec3) -> Sphere>;
// Creates an atom-building closure from the radius and texture of the atoms to be made
// The resulting closure takes the center of the sphere

fn link_builder(f64) -> Box<dyn Fn(Vec3, Vec3) -> EmptyCylinder>;
// Creates a link-building closure from the radius of the links to be made
// The resulting closure takes the centers of the two atoms to join

fn double_builder(f64) -> Box<dyn Fn(Vec3, Vec3, Vec3) -> [EmptyCylinder; 2]>;
// Same with a double link

fn triple_builder(f64) -> Box<dyn Fn(Vec3, Vec3, Vec3) -> [EmptyCylinder; 3]>;
// Same with a triple link

pub fn dimensions(f64) -> [f64; 5];
// Recommended relative sizes for:
//     - radius of C/N/O/...
//     - radius of H
//     - radius of link
//     - length of R-R where R is not H
//     - length of R-H


impl Molecule {
    fn directions(&self) -> [Vec3; 7];              // Create standard possible directions for links (refer to `misc/molecule-orientation-cheatsheet.png`)
    pub fn cyplohexanol(self) -> MoleculeObject;
    pub fn water(self) -> MoleculeObject;
    pub fn methane(self) -> MoleculeObject;
    pub fn ethanol(self) -> MoleculeObject;
    pub fn carbon_dioxide(self) -> MoleculeObject;
    pub fn dinitrogen(self) -> MoleculeObject;
    pub fn benzene(self) -> MoleculeObject;
    pub fn test(self) -> MoleculeObject;            // For debugging purposes
}

impl MoleculeObject {
    pub fn build(self) -> Composite;
}
```
