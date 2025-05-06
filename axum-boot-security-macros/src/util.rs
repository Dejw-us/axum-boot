use std::collections::VecDeque;

use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, FnArg, ItemFn, Lit, PatLit, Type, TypePath, parse_macro_input, parse_quote};
use syn::{ExprLit, LitStr, Token};

pub fn has_user_roles(input: &ItemFn) -> bool {
  input.sig.inputs.iter().any(|arg| {
    if let FnArg::Typed(pat_type) = arg {
      if let Type::Path(TypePath { path, .. }) = &*pat_type.ty {
        if let Some(segment) = path.segments.last() {
          if segment.ident == "UserRolesExtractor" {
            return true;
          }
        }
      }
    }
    false
  })
}

pub fn insert_roles_fn_param_if_missing(input: &mut ItemFn) {
  if !has_user_roles(input) {
    let new_arg =
      parse_quote! { __USER_ROLES__: axum_boot_security::user::extract::UserRolesExtractor };
    input.sig.inputs.insert(0, new_arg);
  }
}

pub fn attr_into_str_vec(attr: TokenStream) -> Result<Vec<String>, TokenStream> {
  let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
  let parsed = match parser.parse(attr) {
    Ok(p) => p,
    Err(err) => return Err(err.to_compile_error().into()),
  };
  let vec = parsed.into_iter().map(|s| s.value()).collect();
  Ok(vec)
}
