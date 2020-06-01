use crate::internal::*;
use std::sync::Arc;

/// Any object that is to be added to a scene needs to either implement this trait or
/// be able to be decomposed into a vector of objects that do.
pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray) -> HitRecord;
    fn texture(&self) -> Texture;
    fn inside(&self, pos: Vec3) -> bool;
}

/// Records information on the surface with which the ray was calculated to intersect.
#[derive(Clone, Copy)]
pub struct ActiveHit {
    /// More of a distance to the hit than an actual time
    pub t: f64,
    /// Position of the intersection between the ray and any object
    pub pos: Vec3,
    /// Normal vector to the surface
    pub normal: Vec3,
    /// Texture of the intersected surface
    pub texture: Texture,
}

/// Add a constant to t in order to record the total length traveled by the ray from a
/// certain point.
impl ActiveHit {
    pub fn later(self, t: f64) -> Self {
        ActiveHit {
            t: self.t + t,
            ..self
        }
    }
}

/// Either no intersection was found, or we have information on the intersection.
pub enum HitRecord {
    Blank,
    Hit(ActiveHit),
}

impl HitRecord {
    pub fn make(t: f64, pos: Vec3, normal: Vec3, texture: Texture) -> Self {
        HitRecord::Hit(ActiveHit {
            t,
            pos,
            normal: normal.unit(),
            texture,
        })
    }

    /// Update self with the contents of other if other is an intersection that occured
    /// earlier that self.
    pub fn compare(&mut self, other: Self) {
        match other {
            HitRecord::Blank => (),
            HitRecord::Hit(b) => match self {
                HitRecord::Blank => *self = other,
                HitRecord::Hit(a) => {
                    if a.t > b.t {
                        *self = other;
                    }
                }
            },
        }
    }
}

/// Wrapper around all objects that can be added to a scene
pub struct Primitive(pub Arc<dyn Hit>);

impl Clone for Primitive {
    fn clone(&self) -> Self {
        Primitive(self.0.clone())
    }
}

impl Primitive {
    pub fn wrap(self) -> Interaction {
        Interaction(vec![self], vec![])
    }

    pub fn intersect(self, other: Self) -> Interaction {
        Interaction(vec![self, other], vec![])
    }

    pub fn remove(self, other: Self) -> Interaction {
        Interaction(vec![self], vec![other])
    }

    pub fn texture(&self) -> Texture {
        self.0.texture()
    }

    pub fn hit(&self, r: &Ray) -> HitRecord {
        self.0.hit(r)
    }

    pub fn inside(&self, pos: Vec3) -> bool {
        self.0.inside(pos)
    }
}

/// A single indivisible object that can be added to the scene without being decomposed.
///
/// Each `Interaction` is a series of restrictions
/// in the form of 'Inside of A' (first vector) or 'Outside of B' (second vector).
#[derive(Clone)]
pub struct Interaction(pub Vec<Primitive>, pub Vec<Primitive>);

impl Interaction {
    /// An easy way of checking that a point is inside an object
    ///
    /// Verify that there exist intersections with the object of two rays that
    /// have the same origin and opposite directions.
    ///
    /// Some specific objects may provide a less costly way to make this test.
    pub fn bidir_hit<T: Hit>(obj: &T, pos: Vec3, v: Vec3) -> bool {
        let ray1 = Ray { orig: pos, dir: v };
        let ray2 = Ray { orig: pos, dir: -v };
        match (obj.hit(&ray1), obj.hit(&ray2)) {
            (HitRecord::Blank, _) => false,
            (_, HitRecord::Blank) => false,
            (_, _) => true,
        }
    }

    /// Wrapper around the `inside` method included in the `Hit` trait.
    pub fn inside(obj: &Primitive, pos: Vec3) -> bool {
        obj.inside(pos)
    }

    /// Wrapper around the `inside` method included in the `Hit` trait.
    pub fn outside(obj: &Primitive, pos: Vec3) -> bool {
        !Interaction::inside(obj, pos)
    }

    /// Add an object to the list of those inside of which a position should be to be considered
    /// inside the interaction.
    pub fn intersect(mut self, other: Primitive) -> Self {
        self.0.push(other);
        self
    }

    /// Add an object to the list of those outside of which a position should be to be considered
    /// inside the interaction.
    pub fn remove(mut self, other: Primitive) -> Self {
        self.1.push(other);
        self
    }

    /// Add an object to the list of those inside of which a position should be to be considered
    /// inside the interaction.
    pub fn intersect_mut(&mut self, other: Primitive) {
        self.0.push(other);
    }

    /// Add an object to the list of those outside of which a position should be to be considered
    /// inside the interaction.
    pub fn remove_mut(&mut self, other: Primitive) {
        self.1.push(other);
    }

    /// The inside/outside test applied to the object that was hit may be unreliable, thus the
    /// final test is done on all but one of the items.
    ///
    /// Use `all_inside_except(p, v, v.len())` to test on all items.
    pub fn all_inside_except(p: Vec3, v: &[Primitive], i: usize) -> bool {
        for (j, item) in v.iter().enumerate() {
            if j != i && Interaction::outside(item, p) {
                return false;
            }
        }
        true
    }

    /// The inside/outside test applied to the object that was hit may be unreliable, thus the
    /// final test is done on all but one of the items.
    ///
    /// Use `all_outside_except(p, v, v.len())` to test on all items.
    pub fn all_outside_except(p: Vec3, v: &[Primitive], i: usize) -> bool {
        for (j, item) in v.iter().enumerate() {
            if j != i && Interaction::inside(item, p) {
                return false;
            }
        }
        true
    }
}
/// A collection of `Interaction`s
///
/// The library can only manage set operations on single objects written as
/// `Union[n=1 to N] ( (Intersection[i=1 to I] A_n,i) \ (Union[j=1 to J] B_n,j) )`.
///
/// PyTrace shows that this is not a significant restriction, as any arbitrary set operation
/// can be expressed in this canonical form (see `pytrace::external::interaction::canonical()`).
pub type Composite = Vec<Interaction>;

/// These are uniform textures that can be set for any object.
#[derive(Clone, Copy)]
pub enum Texture {
    Lambertian(RGB),
    Metal(RGB, f64),
    Light(RGB),
    Dielectric(RGB, f64),
}
