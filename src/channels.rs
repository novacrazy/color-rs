//! Defines fixed-length color channel containers
//!
//! This defines a array-based C-compatible representation for color channels and implements common
//! algebraic operations on them.
//!
//! Such algebraic operations are color-space independent,
//! and should not be used directly unless you want that.

use num_traits::{NumCast, Num, Float};

use typenum::consts::{U1, U2, U3, U4};
use numeric_array::NumericArray;

/// Defines shared behavior for all color channels.
pub trait Channel: Num + Copy + NumCast {
    type FloatChannel: Channel + Float;

    fn into_float(self) -> Self::FloatChannel;
    fn from_float(channel: Self::FloatChannel) -> Self;
}

pub type FloatChannel<C> = <C as Channel>::FloatChannel;

pub trait ChannelAssertion {
    type Channel: Channel;
}

impl<C> ChannelAssertion for C
where
    C: Channel,
{
    type Channel = C;
}

pub type SingleChannel<C> = NumericArray<<C as ChannelAssertion>::Channel, U1>;
pub type DualChannel<C> = NumericArray<<C as ChannelAssertion>::Channel, U2>;
pub type TripleChannel<C> = NumericArray<<C as ChannelAssertion>::Channel, U3>;
pub type QuadChannel<C> = NumericArray<<C as ChannelAssertion>::Channel, U4>;

macro_rules! impl_channel {
    ($($t:ty as $f:ty),*) => {
        $(
            impl Channel for $t {
                type FloatChannel = $f;

                #[inline]
                fn into_float(self) -> Self::FloatChannel {
                    self as $f * <$t>::max_value() as $f
                }

                #[inline]
                fn from_float(f: Self::FloatChannel) -> Self {
                    (f / <$t>::max_value() as $f) as $t
                }
            }
        )*
    }
}

impl Channel for f32 {
    type FloatChannel = f32;

    #[inline(always)]
    fn into_float(self) -> f32 {
        self
    }

    #[inline(always)]
    fn from_float(f: f32) -> f32 {
        f
    }
}

impl Channel for f64 {
    type FloatChannel = f64;

    #[inline(always)]
    fn into_float(self) -> f64 {
        self
    }

    #[inline(always)]
    fn from_float(f: f64) -> f64 {
        f
    }
}

impl_channel! {
    u8 as f32,
    u16 as f32,
    u32 as f32,
    u64 as f64,
    i8 as f32,
    i16 as f32,
    i32 as f32,
    i64 as f64,
    usize as f64,
    isize as f64
}
