#![warn(missing_docs)]

//! A helper crate for nannou drawings.

/// Module that deals with the drawings and the canvas
pub mod draw;
/// Module to easily setup an application given a [Model]
pub mod setup;

pub use draw::Drawable;
pub use setup::setup;
