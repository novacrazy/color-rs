//! The CIE 1931 Yxy (xyY) color space.

use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result as FmtResult};

use num_traits::{Zero, Float};

use color::*;
use channels::*;
use alpha::Alpha;
use white_point::WhitePoint;

declare_color_formats_with_components_plus_alpha_specialization! {
    ///The CIE 1931 Yxy (xyY) color space.
    ///
    ///Yxy is a luminance-chromaticity color space derived from the CIE XYZ
    ///color space. It is widely used to define colors. The chromacity diagrams
    ///for the color spaces are a plot of this color space's x and y coordiantes.
    ///
    ///Conversions and operations on this color space depend on the white point.
    struct Yxy : TripleChannel => YXY {
        ///x chromacity co-ordinate derived from XYZ color space as X/(X+Y+Z).
        ///Typical range is between 0 and 1
        pub x,
        ///y chromacity co-ordinate derived from XYZ color space as Y/(X+Y+Z).
        ///Typical range is between 0 and 1
        pub y,
        ///luma (Y) was a measure of the brightness or luminance of a color.
        ///It is the same as the Y from the XYZ color space. Its range is from
        ///0 to 1, where 0 is black and 1 is white.
        pub luma,
    }
}

pub type Yxya<C, Wp> = Alpha<Yxy<C, Wp>>;

use ::spaces::all::*;

impl<C: Channel, Wp> Default for Yxy<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn default() -> Yxy<C, Wp> {
        let YXY { y, x, .. } = *Yxy::from(Wp::get_xyz());

        Yxy::with_wp(x, y, C::zero())
    }
}

impl<C: Channel, Wp> From<Xyz<C, Wp>> for Yxy<C, Wp>
where
    Wp: WhitePoint<C>,
{
    fn from(xyz: Xyz<C, Wp>) -> Yxy<C, Wp> {
        let mut yxy = Yxy::with_wp(C::zero(), C::zero(), xyz.y);

        let sum: <C as Channel>::FloatChannel = xyz.channels()
            .iter()
            .map(|c| c.into_float())
            .fold(Zero::zero(), |a, b| a + b);

        if sum.is_normal() {
            yxy.x = Channel::from_float(xyz.x.into_float() / sum);
            yxy.y = Channel::from_float(xyz.y.into_float() / sum);
        }

        yxy
    }
}
