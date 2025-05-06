use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemFn, parse_macro_input};

use crate::util::insert_roles_fn_param_if_missing;

pub fn impl_authenticated(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as ItemFn);

  insert_roles_fn_param_if_missing(&mut input);

  TokenStream::from(quote! { #input })
}
