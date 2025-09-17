use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(Service, attributes(service))]
pub fn service_derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let struct_name = input.ident;

  // Look for #[service("path.to.field")]
  let mut service_path: Option<String> = None;

  for attr in &input.attrs {
    if attr.path().is_ident("service") {
      let lit: LitStr = attr
        .parse_args()
        .expect("Expected #[service(\"field_path\")]");
      service_path = Some(lit.value());
    }
  }

  let service_path = service_path.expect("Expected #[service(\"...\")]");

  let expanded = quote! {
      impl ServiceAccessor<#struct_name> for AppState {
          fn get_service(&self) -> std::sync::Arc<#struct_name> {
              self.#service_path.clone()
          }
      }
  };

  expanded.into()
}
