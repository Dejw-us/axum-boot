use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn impl_fn_authorizer(_attr: TokenStream, input: ItemFn) -> TokenStream {
  let fn_name = input.sig.ident;
  let fn_body = input.block;

  quote! {
    #[allow(non_camel_case_types)]
    struct #fn_name;

    impl<S> axum::extract::FromRequestParts<S> for #fn_name {
      type Rejection = axum::http::StatusCode;

      fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
      ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
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
