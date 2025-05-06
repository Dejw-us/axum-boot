use proc_macro::TokenStream;
use quote::quote;
use syn::{Block, ItemFn, parse_quote};

use crate::util::{attr_into_str_vec, insert_roles_fn_param_if_missing};

pub fn impl_roles_macro(attr: TokenStream, mut input: ItemFn) -> TokenStream {
  let roles = match attr_into_str_vec(attr) {
    Ok(roles) => roles,
    Err(err) => return err,
  };

  insert_roles_fn_param_if_missing(&mut input);

  let checks = roles
    .iter()
    .map(|r| {
      quote! {
        if !__USER_ROLES__.0.has_role(#r) {
          return Err(axum::http::StatusCode::UNAUTHORIZED)
        }
      }
      .into()
    })
    .collect::<Vec<proc_macro2::TokenStream>>();

  let old_fn_block = &input.block;

  let new_fn_block = parse_quote!({
    #(#checks)*
    #old_fn_block
  });

  input.block = Box::new(new_fn_block);

  quote! {#input}.into()
}
