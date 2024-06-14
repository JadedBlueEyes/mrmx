//! # mrmx-macros
//!
//! This crate contains macros for generating [`mrml`](https://docs.rs/mrml/) using a JSX-like syntax.
//!
//! ```
//! # use mrmx_macros::view;
//! view! { <mj-title>title</mj-title> };
//! ```
//!
//! Please see the `mrmx` crate for more information.
mod view;
use quote::quote;

#[proc_macro_error::proc_macro_error]
#[proc_macro]
#[cfg_attr(debug_assertions, tracing::instrument(level = "trace", skip_all))]
pub fn view(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();

    let config = rstml::ParserConfig::default().recover_block(true);
    let parser = rstml::Parser::new(config);
    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());
    let nodes_output = view::render_view(&nodes);
    quote! {
        {
            #(#errors;)*
            #nodes_output
        }
    }
    .into()
}
