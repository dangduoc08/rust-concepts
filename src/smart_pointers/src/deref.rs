use super::custom_box::CustomBox;
use std::ops::Deref;

impl<T> Deref for CustomBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
