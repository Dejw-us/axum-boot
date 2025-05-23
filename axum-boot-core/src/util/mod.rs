#[macro_export]
macro_rules! string {
  () => {
    String::new()
  };
  ($val:expr) => {
    String::from($val)
  };
}

pub fn if_else<T>(condition: bool, if_true: T, if_false: T) -> T
where
{
  if condition { if_true } else { if_false }
}
