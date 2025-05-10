use axum::http::request::Parts;
use axum_boot_security::{macros::authorizer, user::role::UserRoles};

use crate::app::AppState;

struct TestAuthorizer;

#[authorizer]
pub async fn check(parts: &Parts) -> bool {
  let roles = UserRoles::from_parts(parts);
  let app_state = parts.extensions.get::<AppState>().unwrap();
  println!("state: {:?}", app_state);
  roles.has_role("user")
}
