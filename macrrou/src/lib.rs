mod launch_nannou_app;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::launch_nannou_app::NannouApp;

#[proc_macro]
pub fn launch_nannou_app(input: TokenStream) -> TokenStream {
    let nannou_app = parse_macro_input!(input as NannouApp);

    nannou_app.generate().into()
}
