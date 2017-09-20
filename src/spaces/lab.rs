use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result as FmtResult};

use num_traits::{One, Float};

use color::*;
use channels::*;
use alpha::Alpha;
use white_point::WhitePoint;


declare_color_formats_with_components_plus_alpha_specialization! {
    /// The CIE L*a*b* (CIELAB) color space.
    ///
    /// CIE L*a*b* is a device independent color space which includes all
    /// perceivable colors. It's sometimes used to convert between other color
    /// spaces, because of its ability to represent all of their colors, and
    /// sometimes in color manipulation, because of its perceptual uniformity. This
    /// means that the perceptual difference between two colors is equal to their
    /// numerical difference.
    ///
    /// The parameters of L*a*b* are quite different, compared to many other color
    /// spaces, so manipulating them manually may be unintuitive.
    struct Lab : TripleChannel => LAB {
        /// L* is the lightness of the color. 0.0 gives absolute black and 100
        /// give the brightest white.
        pub l,
        /// a* goes from red at -128 to green at 127.
        pub a,
        /// b* goes from yellow at -128 to blue at 127.
        pub b,
    }
}

pub type Laba<C, Wp> = Alpha<Lab<C, Wp>>;

use ::spaces::all::*;

impl<C: Channel, Wp> Default for Lab<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn default() -> Lab<C, Wp> {
        unimplemented!()
    }
}

impl<C: Channel, Wp> From<Xyz<C, Wp>> for Lab<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn from(xyz: Xyz<C, Wp>) -> Lab<C, Wp> {
        unimplemented!()
    }
}

impl<C: Channel, Wp> From<Yxy<C, Wp>> for Lab<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn from(yxy: Yxy<C, Wp>) -> Lab<C, Wp> {
        unimplemented!()
    }
}