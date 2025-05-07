use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, Ident, ItemFn, Pat, PatIdent, Type, TypeReference};

pub fn impl_fn_authorizer(_attr: TokenStream, input: ItemFn) -> TokenStream {
  let fn_name = input.sig.ident;
  let fn_body = input.block;
  let inputs = input
    .sig
    .inputs
    .iter()
    .filter_map(|arg| {
      if let FnArg::Typed(pat) = arg {
        if let Pat::Ident(PatIdent { ident, .. }) = *pat.pat.clone() {
          return Some((ident, &*pat.ty));
        }
      }
      None
    })
    .collect::<Vec<_>>();
  let parts_name = inputs.iter().find_map(|(ident, ty)| match ty {
    Type::Path(type_path) => {
      let is_parts = type_path
        .path
        .segments
        .last()
        .map_or(false, |seg| seg.ident == "Parts");
      if is_parts { Some(ident.clone()) } else { None }
    }
    Type::Reference(TypeReference { elem, .. }) => {
      if let Type::Path(type_path) = &**elem {
        let is_parts = type_path
          .path
          .segments
          .last()
          .map_or(false, |seg| seg.ident == "Parts");
        if is_parts { Some(ident.clone()) } else { None }
      } else {
        None
      }
    }
    _ => None,
  });
  let parts_name = parts_name.unwrap_or(format_ident!("parts"));
  let vars = inputs
    .iter()
    .filter(|(ident, _)| ident.to_string() != parts_name.to_string())
    .map(|(ident, ty)| {
      quote! {let #ident: #ty = #parts_name.extensions.get::<#ty>();}
    })
    .collect::<Vec<_>>();

  quote! {
    #[allow(non_camel_case_types)]
    struct #fn_name;

    impl<S> axum::extract::FromRequestParts<S> for #fn_name {
      type Rejection = axum::http::StatusCode;

      fn from_request_parts(
        #parts_name: &mut Parts,
        __STATE__: &S,
      ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
          if #parts_name.extensions.get::<Arc<UserRoles>>().is_none() {
            return Err(axum::http::StatusCode::FORBIDDEN)
          }
          let is_authorized = {
            #fn_body
          };
          if is_authorized {
            Ok(#fn_name)
          } else {
            Err(axum::http::StatusCode::UNAUTHORIZED)
          }
        }
      }
    }
  }
  .into()
}
