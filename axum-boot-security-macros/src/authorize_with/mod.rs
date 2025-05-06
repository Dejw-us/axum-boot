use core::arch;
use std::alloc::System;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ExprArray, ExprClosure, ItemFn, Type, parse_macro_input, parse_quote};

pub fn impl_authorize_with(attr: TokenStream, mut input: ItemFn) -> TokenStream {
  let ty = parse_macro_input!(attr as Type);
  let param_name = format_ident!("_{}", quote!(#ty).to_string().to_lowercase());
  let new_arg = parse_quote! {#param_name: #ty};

  input.sig.inputs.insert(0, new_arg);

  quote! {#input}.into()
}
