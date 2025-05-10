use std::{borrow::Cow, sync::Arc};

use axum::http::request::Parts;

use super::extract::UserRolesExtractor;

#[derive(Clone)]
pub struct UserRoles(pub(crate) Vec<String>);

impl UserRoles {
  pub fn from_parts(parts: &Parts) -> Cow<'_, Self> {
    if let Some(roles) = parts.extensions.get::<Arc<UserRoles>>() {
      Cow::Borrowed(roles.as_ref())
    } else {
      Cow::Owned(UserRoles(vec![]))
    }
  }

  pub fn has_role(&self, role: &str) -> bool {
    self.0.contains(&role.to_string())
  }
}
