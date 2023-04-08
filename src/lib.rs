#![warn(missing_docs)]

//! A helper crate for nannou drawings.

/// Module that deals with the drawings and the canvas
pub mod draw;
/// Module to easily setup an application given a [setup::Model]
pub mod setup;

/// Typical symbols that you'd want to include from this crate
pub mod prelude {
    pub use crate::draw::{self, *};
    pub use crate::setup::{self, *};
}
