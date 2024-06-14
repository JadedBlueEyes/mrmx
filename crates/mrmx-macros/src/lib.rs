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
