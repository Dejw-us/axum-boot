use crate::string;

pub enum Error {
  Message(String),
  Empty,
}

impl Error {
  pub fn map_message<T>(message: &str) -> impl Fn(T) -> Self {
    move |_| Error::Message(string!(message))
  }

  pub fn map_empty<T>() -> impl Fn(T) -> Self {
    move |_| Error::Empty
  }
}
