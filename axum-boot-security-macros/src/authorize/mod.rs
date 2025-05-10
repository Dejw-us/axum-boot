use parser::ParsedAttr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

pub mod parser;

pub fn impl_authorize(attr: TokenStream, mut input: ItemFn) -> TokenStream {
  let ParsedAttr { authorizers } = parse_macro_input!(attr as ParsedAttr);

  let fn_param = parse_quote! {
    __HANDLER_AUTHORIZATION__: axum_boot_security::authorization::HandlerAuthorization<(#(#authorizers),*,)>
  };

  input.sig.inputs.insert(0, fn_param);

  quote! {#input}.into()
}
