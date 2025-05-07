use authenticated::impl_authenticated;
use authorize_with::impl_authorize_with;
use authorizer::impl_fn_authorizer;
use proc_macro::TokenStream;
use roles::impl_roles_macro;
use syn::{ItemFn, parse_macro_input};

mod authenticated;
mod authorize_with;
mod authorizer;
mod roles;
mod util;

#[proc_macro_attribute]
pub fn authenticated(attr: TokenStream, input: TokenStream) -> TokenStream {
  impl_authenticated(attr, input)
}

#[proc_macro_attribute]
pub fn roles(attr: TokenStream, input: TokenStream) -> TokenStream {
  impl_roles_macro(attr, parse_macro_input!(input as ItemFn))
}

#[proc_macro_attribute]
pub fn authorize_with(attr: TokenStream, input: TokenStream) -> TokenStream {
  impl_authorize_with(attr, parse_macro_input!(input as ItemFn))
}

#[proc_macro_attribute]
pub fn authorizer(attr: TokenStream, input: TokenStream) -> TokenStream {
  impl_fn_authorizer(attr, parse_macro_input!(input as ItemFn))
}
