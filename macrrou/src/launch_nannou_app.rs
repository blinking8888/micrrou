use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Expr, ExprAssign, Ident, LitInt, Meta, Token};

#[derive(Debug)]
pub struct NannouApp {
    model: Ident,
    width: LitInt,
    height: LitInt,
}

const CANVAS_MISSING_EXAMPLE: &str = r#"Example:
launch_nannou_app!(for Model, canvas(width=900, height=900))
                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
"#;

const MISSING_WIDTH_MSG: &str = r#"Missing param 'width' param.
Example: launch_nannou_app!(for Model, canvas(width=900, height=900))
                                              ^^^^^^^^^ <- this is missing
"#;

const MISSING_HEIGHT_MSG: &str = r#"Missing param 'height' param.
Example: launch_nannou_app!(for Model, canvas(width=900, height=900))
                                                         ^^^^^^^^^^ <- this is missing
"#;

/// launch_nannou_app!(for ModelData, canvas(width=900, height=900));
impl syn::parse::Parse for NannouApp {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let _for: Token![for] = input.parse()?;
        let model: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let canvas: Meta = input.parse()?;

        if !canvas.path().is_ident("canvas") {
            let msg = format!(
                "`canvas' identifier is missing found `{}'\n{}",
                canvas.path().get_ident().unwrap(),
                CANVAS_MISSING_EXAMPLE
            );
            return Err(syn::Error::new(canvas.span(), msg));
        }

        let mut width: Option<LitInt> = None;
        let mut height: Option<LitInt> = None;

        if let Meta::List(ref canvas) = &canvas {
            let args = canvas
                .parse_args_with(Punctuated::<syn::ExprAssign, Token![,]>::parse_terminated)?;

            for arg in args.iter() {
                if width.is_none() {
                    width = get_lit_int_for_ident("width", arg);
                }

                if height.is_none() {
                    height = get_lit_int_for_ident("height", arg);
                }
            }
        }

        let width = width.ok_or(syn::Error::new(canvas.span(), MISSING_WIDTH_MSG))?;
        let height = height.ok_or(syn::Error::new(canvas.span(), MISSING_HEIGHT_MSG))?;

        Ok(NannouApp {
            model,
            width,
            height,
        })
    }
}

impl NannouApp {
    pub fn generate(self) -> proc_macro2::TokenStream {
        let model = self.model;
        let width = self.width;
        let height = self.height;
        quote! {
            micrrou::nannou_app::launch::<#model, #width, #height>();
        }
    }
}

fn get_lit_int_for_ident(ident: &'static str, expr: &ExprAssign) -> Option<LitInt> {
    if let Expr::Path(ref path) = *expr.left {
        if path.path.is_ident(ident) {
            if let Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref value),
                ..
            }) = *expr.right
            {
                Some(value.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
