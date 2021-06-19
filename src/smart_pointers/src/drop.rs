use super::custom_box::CustomBox;

impl<T> Drop for CustomBox<T> {
  fn drop(&mut self) {
    println!("Custom Box dropping")
  }
}
