//! The CIE 1931 XYZ color space.

use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result as FmtResult};

use num_traits::{Zero, One, Float};

use color::*;
use channels::*;
use alpha::Alpha;
use white_point::WhitePoint;

declare_color_formats_with_components_plus_alpha_specialization! {
    /// The CIE 1931 XYZ color space
    ///
    /// XYZ links the perceived colors to their wavelengths and simply makes it
    /// possible to describe the way we see colors as numbers. It's often used when
    /// converting from one color space to an other, and requires a standard
    /// illuminant and a standard observer to be defined.
    ///
    /// Conversions and operations on this color space depend on the defined white point
    struct Xyz : TripleChannel => XYZ {
        /// X is the scale of what can be seen as a response curve for the cone
        /// cells in the human eye. Its range depends on the white point and goes
        /// from 0.0 to 0.95047 for the default D65.
        pub x,
        /// Y is the luminance of the color, where 0.0 is black and 1.0 is white.
        pub y,
        /// Z is the scale of what can be seen as the blue stimulation. Its range depends
        /// on the white point and goes from 0.0 to 1.08883 for the default D65.
        pub z,
    }
}

pub type Xyza<C, Wp> = Alpha<Xyz<C, Wp>>;

use ::spaces::all::*;

impl<C: Channel, Wp> Default for Xyz<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn default() -> Xyz<C, Wp> {
        Xyz::with_wp(C::zero(), C::zero(), C::zero())
    }
}

impl<C: Channel, Wp> From<Yxy<C, Wp>> for Xyz<C, Wp> where Wp: WhitePoint<C> {
    fn from(yxy: Yxy<C, Wp>) -> Xyz<C, Wp> {
        let yxy = yxy.into_float();

        let mut xyz = Xyz::raw(Zero::zero(), yxy.luma, Zero::zero());

        if yxy.y.is_normal() {
            xyz.x = yxy.luma * yxy.x / yxy.y;
            xyz.z = yxy.luma * ( FloatChannel::<C>::one() - yxy.x - yxy.y ) / yxy.y;
        }

        Xyz::from_float(xyz)
    }
}