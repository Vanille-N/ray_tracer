macro_rules! repr {
    ( $t:tt, $x:expr ) => {
        $t::from($x).__repr__().ok().unwrap()
    };
    ( $x:expr ) => {
        $x.__repr__().ok().unwrap()
    };
}

mod camera;
mod cfg;
mod composites;
mod interaction;
mod primitives;
mod sky;
mod texture;
mod vec;

pub use camera::Camera;
pub use cfg::Cfg;
pub use composites::{Axes, Cradle, Prebuilt};
pub use interaction::{Construct, Primitive, ToInternal};
pub use primitives::*;
pub use sky::Sky;
pub use texture::{Texture, RGB};
pub use vec::Vec;
