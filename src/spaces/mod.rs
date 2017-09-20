//pub mod rgb;
pub mod xyz;
pub mod yxy;
pub mod lab;

pub mod all {
    pub use super::xyz::Xyz;
    pub use super::yxy::Yxy;
    //pub use super::lab::Lab;
}