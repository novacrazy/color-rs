#![allow(unused_macros, unused_imports)]

extern crate num_traits;
extern crate typenum;
#[macro_use]
extern crate generic_array;
#[macro_use]
extern crate numeric_array;
extern crate nalgebra;

pub mod channels;
#[macro_use]
pub mod color;
pub mod alpha;
//pub mod limited;
pub mod white_point;
pub mod spaces;
//pub mod blend;
//pub mod gamma;

pub mod prelude {
    //pub use ::channels::Channel;
    //pub use ::color::Color;
    //pub use ::blend::{Blend, BlendSync, GenericBlend, BoxedGenericBlend, BoxedGenericBlendSync};
    //pub use ::formats::rgb;
    //pub use ::gamma;
}
