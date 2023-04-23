#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! A helper crate for nannou drawings.

/// Module that deals with the drawings and the canvas
pub mod draw;
/// Module to easily setup an application given a [nannou_app::Model]
pub mod nannou_app;

/// Typical symbols that you'd want to include from this crate
pub mod prelude {
    pub use crate::draw::{self, *};
    pub use crate::nannou_app::{self, *};
}

#[cfg(feature = "macro")]
pub use macrrou::launch_nannou_app;
