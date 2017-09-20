use numeric_array::NumericSequence;

use channels::*;
use white_point::WhitePoint;

pub trait Color {
    /// The type of each color channel
    type Channel: Channel;

    /// Container type for the color channels
    type Channels: NumericSequence<Self::Channel>;

    /// Associated whitepoint type
    type WhitePoint: WhitePoint<Self::Channel>;

    /// Create a new color from its raw channel data
    fn from_channels(channels: Self::Channels) -> Self;

    /// Returns a reference to the internal color channels
    fn channels(&self) -> &Self::Channels;

    /// Returns a mutable reference to the internal color channels
    fn channels_mut(&mut self) -> &mut Self::Channels;
}

pub trait Components {
    type Components;

    fn as_components(&self) -> &Self::Components;
    fn as_components_mut(&mut self) -> &mut Self::Components;
}

/// Convenience type alias
pub type ColorChannel<C> = <C as Color>::Channel;

/// Convenience type alias
pub type ColorChannels<C> = <C as Color>::Channels;

/// Convenience type alias
pub type ColorWhitePoint<C> = <C as Color>::WhitePoint;

/// Convenience type alias
pub type ColorComponents<C> = <C as Components>::Components;

pub trait FromColor<C: Channel, Wp> where Wp: WhitePoint<C> {
    fn from_xyz(xyz: ::spaces::xyz::Xyz<C, Wp>) -> Self;
    fn from_yxy(yxy: ::spaces::yxy::Yxy<C, Wp>) -> Self;
}

macro_rules! declare_color_components {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident : $channels:ident => $component:ident { $(
            $(#[$($component_attrs:tt)*])*
            pub $c:ident,
        )* }
    ),*) => {
        /// Color components
        pub mod components {
            $(
                /// Color components
                #[repr(C)]
                #[derive(Debug, PartialEq, Eq, Hash)]
                pub struct $component<C> {
                    $(
                        $(#[$($component_attrs)*])*
                        pub $c: C,
                    )*
                }

                impl<C: Copy> Clone for $component<C> {
                    fn clone(&self) -> $component<C> {
                        $component { ..*self }
                    }
                }

                impl<C: Copy> Copy for $component<C> {}
            )*
        }

        $(
            use self::components::$component;

            impl<C: Channel, Wp> Deref for $name<C, Wp> {
                type Target = $component<C>;

                #[inline]
                fn deref(&self) -> &$component<C> {
                    self.as_components()
                }
            }

            impl<C: Channel, Wp> DerefMut for $name<C, Wp> {
                #[inline]
                fn deref_mut(&mut self) -> &mut $component<C> {
                    self.as_components_mut()
                }
            }

            impl<C: Channel, Wp> From<$component<C>> for $name<C, Wp>
            where
                Wp: WhitePoint<C>
            {
                fn from(components: $component<C>) -> $name<C, Wp> {
                    let $component { $($c,)* } = components;

                    $name::with_wp($($c,)*)
                }
            }

            impl<C: Channel, Wp> Components for $name<C, Wp> {
                type Components = $component<C>;

                #[inline]
                fn as_components(&self) -> &Self::Components {
                    unsafe { ::std::mem::transmute(self) }
                }

                #[inline]
                fn as_components_mut(&mut self) -> &mut Self::Components {
                    unsafe { ::std::mem::transmute(self) }
                }
            }
        )*
    }
}

macro_rules! declare_color_format {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident : $channels:ident => $component:ident { $(
            $(#[$($component_attrs:tt)*])*
            pub $c:ident,
        )* }
    ),*) => {
        $(
            $(#[$($attrs)*])*
            #[repr(C)]
            pub struct $name<C: Channel = f32, Wp = ::white_point::D65> {
                channels: $channels<C>,
                white_point: PhantomData<Wp>
            }

            impl<C: Channel, Wp> Debug for $name<C, Wp>
            where
                C: Debug
            {
                fn fmt(&self, f: &mut Formatter) -> FmtResult {
                    f.debug_struct(stringify!($name)).field("channels", &self.channels).finish()
                }
            }

            impl<C: Channel, Wp> Clone for $name<C, Wp> {
                fn clone(&self) -> $name<C, Wp> {
                    $name { ..*self }
                }
            }

            impl<C: Channel, Wp> Copy for $name<C, Wp> {}

            impl<C: Channel> $name<C, ::white_point::D65> {
                #[inline(always)]
                pub fn new($($c: C,)*) -> $name<C, ::white_point::D65> {
                    $name::with_wp($($c,)*)
                }
            }

            // This is just some cheating to allow direct creation internally without caring about the whitepoint,
            // such as when converting between `C` and `FloatChannel<C>`
            impl<C: Channel, Wp> $name<C, Wp> {
                #[inline]
                fn raw($($c: C,)*) -> $name<C, Wp> {
                    use std::mem;

                    debug_assert_eq!(mem::size_of::<$channels<C>>(), mem::size_of_val(&[$($c,)*]));

                    $name {
                        channels: unsafe { mem::transmute_copy(&[$($c,)*]) },
                        white_point: PhantomData,
                    }
                }
            }

            impl<C: Channel, Wp> $name<C, Wp>
            where
                Wp: WhitePoint<C>
            {
                #[inline(always)]
                pub fn with_wp($($c: C,)*) -> $name<C, Wp> {
                    Self::raw($($c,)*)
                }
            }

            impl<C: Channel, Wp> $name<C, Wp> {
                #[inline]
                pub fn into_float(self) -> $name<FloatChannel<C>, Wp> {
                    $name {
                        channels: self.channels.into_array().map(Channel::into_float).into(),
                        white_point: PhantomData,
                    }
                }

                #[inline]
                pub fn from_float(fcolor: $name<FloatChannel<C>, Wp>) -> Self {
                    $name {
                        channels: fcolor.channels.into_array().map(Channel::from_float).into(),
                        white_point: PhantomData
                    }
                }
            }

            impl<C: Channel, Wp> Color for $name<C, Wp>
            where
                Wp: WhitePoint<C>
            {
                type Channel = C;
                type Channels = $channels<C>;
                type WhitePoint = Wp;

                fn from_channels(channels: $channels<C>) -> $name<C, Wp> {
                    $name { channels, white_point: PhantomData }
                }

                fn channels(&self) -> &$channels<C> {
                    &self.channels
                }

                fn channels_mut(&mut self) -> &mut $channels<C> {
                    &mut self.channels
                }
            }

            impl<C: Channel, Wp> FromColor<C, Wp> for $name<C, Wp>
            where
                Wp: WhitePoint<C>
            {
                fn from_xyz(xyz: ::spaces::xyz::Xyz<C, Wp>) -> Self { xyz.into() }
                fn from_yxy(yxy: ::spaces::yxy::Yxy<C, Wp>) -> Self { yxy.into() }
            }
        )*
    }
}

macro_rules! declare_color_formats_with_components {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident : $channels:ident => $component:ident { $(
            $(#[$($component_attrs:tt)*])*
            pub $c:ident,
        )* }
    ),*) => {
        $(
            declare_color_format! {
                $(#[$($attrs)*])*
                struct $name : $channels => $component { $(
                    $(#[$($component_attrs)*])*
                    pub $c,
                )* }
            }

            declare_color_components! {
                $(#[$($attrs)*])*
                struct $name : $channels => $component { $(
                    $(#[$($component_attrs)*])*
                    pub $c,
                )* }
            }
        )*
    }
}

macro_rules! declare_color_formats_with_components_plus_alpha_specialization {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident : $channels:ident => $component:ident { $(
            $(#[$($component_attrs:tt)*])*
            pub $c:ident,
        )* }
    ),*) => {
        $(
            declare_color_formats_with_components! {
                $(#[$($attrs)*])*
                struct $name : $channels => $component { $(
                    $(#[$($component_attrs)*])*
                    pub $c,
                )* }
            }

            impl<C: Channel> Alpha<$name<C>> {
                pub fn new($($c: C,)* alpha: C) -> Alpha<$name<C>> {
                    Alpha::from_color($name::new($($c,)*), alpha)
                }
            }

            impl<C: Channel, Wp> Alpha<$name<C, Wp>>
            where
                Wp: WhitePoint<C>
            {
                pub fn with_wp($($c: C,)* alpha: C) -> Alpha<$name<C, Wp>> {
                    Alpha::from_color($name::with_wp($($c,)*), alpha)
                }
            }
        )*
    }
}

/*
macro_rules! declare_color_newtype_formats {
    ($(
        $(#[$($attrs:tt)*])*
        struct $name:ident = $subtype:ident,
    )*) => {
        $(
            $(#[$($attrs)*])*
            #[repr(C)]
            pub struct $name<C: Channel = f32, Wp = ::white_point::D65>($subtype<C, Wp>);

            impl<C: Channel, Wp> Deref for $name<C, Wp> where Wp: WhitePoint<C> {
                type Target = $subtype<C, Wp>;

                fn deref(&self) -> &$subtype<C, Wp> {
                    &self.0
                }
            }

            impl<C: Channel, Wp> DerefMut for $name<C, Wp> where Wp: WhitePoint<C> {
                fn deref_mut(&mut self) -> &mut $subtype<C, Wp> {
                    &mut self.0
                }
            }

            impl<C: Channel, Wp> Debug for $name<C, Wp> where C: Debug {
                fn fmt(&self, f: &mut Formatter) -> FmtResult {
                    f.debug_struct(stringify!($name)).field("channels", &self.0.channels).finish()
                }
            }

            impl<C: Channel, Wp> Clone for $name<C, Wp> {
                fn clone(&self) -> $name<C, Wp> {
                    $name { ..*self }
                }
            }

            impl<C: Channel, Wp> Copy for $name<C, Wp> {}

            impl<C: Channel, Wp> $name<C, Wp> where Wp: WhitePoint<C> {
                pub fn new(color: $subtype<C, Wp>) -> $name<C, Wp> {
                    $name(color)
                }
            }

            impl<C: Channel, Wp> Color for $name<C, Wp> where Wp: WhitePoint<C> {
                type Channels = <$subtype<C, Wp> as Color>::Channels;
                type WhitePoint = Wp;

                fn channels(&self) -> &Self::Channels {
                    self.0.channels()
                }

                fn channels_mut(&mut self) -> &mut Self::Channels {
                    self.0.channels_mut()
                }
            }
        )*
    }
}
*/
