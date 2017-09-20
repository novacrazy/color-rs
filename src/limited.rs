/// A trait for clamping and checking if colors are within their ranges.
pub trait Limited {
    /// Check if the color's components are within the expected ranges.
    fn is_valid(&self) -> bool;

    /// Return a new color where the components has been
    /// clamped to the nearest valid values.
    fn clamp(&self) -> Self;

    /// Clamp color components in-place
    fn clamp_self(&mut self);
}