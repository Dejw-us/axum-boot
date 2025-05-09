use jsonwebtoken::{Header, decode_header};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Jwk {
  pub kid: Option<String>,
  pub kty: String,
  pub alg: Option<String>,
  #[serde(rename = "use")]
  pub use_: Option<String>,
  pub x5c: Option<Vec<String>>,
  pub x5t: Option<String>,
  #[serde(rename = "x5t#S256")]
  pub x5t_s256: Option<String>,
  pub n: String,
  pub e: String,
  pub key_ops: Option<Vec<String>>,
  pub ext: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Jwks {
  pub keys: Vec<Jwk>,
}

impl Jwks {
  pub async fn fetch(jwks_url: &str) -> reqwest::Result<Jwks> {
    let jwks = reqwest::get(jwks_url).await?.json::<Jwks>().await?;
    Ok(jwks)
  }

  pub fn sig(&self, header: Header) -> Option<&Jwk> {
    if let Some(kid) = header.kid {
      if let Some(jwk) = self.keys.iter().find(|key| {
        let kid_matches = key.kid.as_ref() == Some(&kid);
        let sig_matches = key.use_.as_ref().map_or(true, |u| u == "sig");
        kid_matches && sig_matches
      }) {
        return Some(jwk);
      }
    }
    None
  }
}
