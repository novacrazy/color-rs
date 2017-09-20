//! Defines the tristimulus values of the CIE Illuminants.
//!
//! White point is the reference white or target white as seen by a standard observer under a
//! standard illuminant. For example, photographs taken indoors may be lit by incandescent lights,
//! which are relatively orange compared to daylight. Defining "white" as daylight will give
//! unacceptable results when attempting to color-correct a photograph taken with incandescent lighting.

use std::fmt::{Display, Formatter, Result as FmtResult};

use channels::Channel;
use spaces::xyz::Xyz;

pub trait WhitePoint<C: Channel>: Sized {
    fn get_xyz() -> Xyz<C, Self>;
}

macro_rules! declare_whitepoints {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident { $x:expr, $y:expr, $z:expr },
    )*) => {
        $(
            $(#[$($attrs)*])*
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            pub struct $name;

            impl Display for $name {
                fn fmt(&self, f: &mut Formatter) -> FmtResult {
                    write!(f, "Standard Illuminant {} ({}, {}, {})", stringify!($name), $x, $y, $z)
                }
            }

            impl<C: Channel> WhitePoint<C> for $name {
                fn get_xyz() -> Xyz<C, Self> {
                    Xyz::with_wp(C::from($x).unwrap(),
                                 C::from($y).unwrap(),
                                 C::from($z).unwrap())
                }
            }
        )*
    }
}

declare_whitepoints! {
    /// CIE standard illuminant A
    ///
    /// CIE standard illuminant A is intended to represent typical, domestic, tungsten-filament lighting.
    /// Its relative spectral power distribution is that of a Planckian radiator at a temperature of approximately 2856 K.
    /// Uses the CIE 1932 2° Standard Observer
    struct A { 1.09850, 1.0, 0.35585 },

    /// CIE standard illuminant B
    ///
    /// CIE standard illuminant B represents noon sunlight, with a correlated color temperature (CCT) of 4874 K
    /// Uses the CIE 1932 2° Standard Observer
    struct B { 0.99072, 1.0, 0.85223 },

    /// CIE standard illuminant C
    ///
    /// CIE standard illuminant C represents the average day light with a CCT of 6774 K
    /// Uses the CIE 1932 2° Standard Observer
    struct C { 0.98074, 1.0, 1.18232 },

    /// CIE D series standard illuminant - D50
    ///
    /// D50 White Point is the natural daylight with a color temperature of around 5000K
    /// for 2° Standard Observer.
    struct D50 { 0.96422, 1.0, 0.82521 },

    /// CIE D series standard illuminant - D55
    ///
    /// D55 White Point is the natural daylight with a color temperature of around 5500K
    /// for 2° Standard Observer.
    struct D55 { 0.95682, 1.0, 0.92149 },

    /// CIE D series standard illuminant - D65
    ///
    /// D65 White Point is the natural daylight with a color temperature of 6500K
    /// for 2° Standard Observer.
    struct D65 { 0.95047, 1.0, 1.08883 },

    /// CIE D series standard illuminant - D75
    ///
    /// D75 White Point is the natural daylight with a color temperature of around 7500K
    /// for 2° Standard Observer.
    struct D75 { 0.94972, 1.0, 1.22638 },

    /// CIE standard illuminant E
    ///
    /// CIE standard illuminant E represents the equal energy radiator
    /// Uses the CIE 1932 2° Standard Observer
    struct E { 1.0, 1.0, 1.0 },

    /// CIE fluorescent illuminant series - F2
    ///
    /// F2 represents a semi-broadband fluorescent lamp for 2° Standard Observer.
    struct F2 { 0.99186, 1.0, 0.67393},

    /// CIE fluorescent illuminant series - F7
    ///
    /// F7 represents a broadband fluorescent lamp for 2° Standard Observer.
    struct F7 { 0.95041, 1.0, 1.08747 },

    ///CIE fluorescent illuminant series - F11
    ///
    ///F11 represents a narrowband fluorescent lamp for 2° Standard Observer.
    struct F11 { 1.00962, 1.0, 0.64350 },
}

/// Alternative D-series illuminants for 10° Standard Observer.
pub mod degree10 {
    use super::*;

    declare_whitepoints! {
        ///CIE D series standard illuminant - D50
        ///
        ///D50 White Point is the natural daylight with a color temperature of around 5000K
        ///for 10° Standard Observer.
        struct D50Degree10 { 0.9672, 1.0, 0.8143 },

        ///CIE D series standard illuminant - D55
        ///
        ///D55 White Point is the natural daylight with a color temperature of around 5500K
        ///for 10° Standard Observer.
        struct D55Degree10 { 0.958, 1.0, 0.9093 },

        ///CIE D series standard illuminant - D65
        ///
        ///D65 White Point is the natural daylight with a color temperature of 6500K
        ///for D65Degree10° Standard Observer.
        struct D65Degree10 { 0.9481, 1.0, 1.073 },

        ///CIE D series standard illuminant - D75
        ///
        ///D75 White Point is the natural daylight with a color temperature of around 7500K
        ///for 10° Standard Observer.
        struct D75Degree10 { 0.94416, 1.0, 1.2064 },
    }
}
