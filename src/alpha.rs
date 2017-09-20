use std::ops::{Deref, DerefMut};

use color::{Color, ColorChannel, Components, ColorComponents};

#[repr(C)]
pub struct Alpha<C: Color> {
    color: C,
    pub alpha: ColorChannel<C>,
}

impl<C: Color> Alpha<C> {
    pub fn from_color(color: C, alpha: ColorChannel<C>) -> Alpha<C> {
        Alpha { color, alpha }
    }
}

impl<C: Color> Deref for Alpha<C>
where
    C: Components,
{
    type Target = ColorComponents<C>;

    fn deref(&self) -> &Self::Target {
        self.color.as_components()
    }
}

impl<C: Color> DerefMut for Alpha<C>
where
    C: Components,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.color.as_components_mut()
    }
}

//impl<C: Color> Color for Alpha<C> {
//
//}
