//! macrrou
//! Convenience procedural macro to easily launch an app in the [nannou](https://nannou.cc) framework

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod launch_nannou_app;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::launch_nannou_app::NannouApp;

/// Launches a nannou app
#[proc_macro]
pub fn launch_nannou_app(input: TokenStream) -> TokenStream {
    let nannou_app = parse_macro_input!(input as NannouApp);

    nannou_app.generate().into()
}
