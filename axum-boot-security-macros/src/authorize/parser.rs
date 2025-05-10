use syn::{Token, Type, parse::Parse, punctuated::Punctuated};

pub struct ParsedAttr {
  pub authorizers: Vec<Type>,
}

impl Parse for ParsedAttr {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let parsed = Punctuated::<Type, Token![,]>::parse_terminated(input)?;
    let types = parsed.iter().map(|ty| ty.clone()).collect::<Vec<_>>();
    Ok(Self { authorizers: types })
  }
}
