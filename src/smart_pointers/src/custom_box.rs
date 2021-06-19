#[derive(Debug)]
pub struct CustomBox<T>(pub T);

impl<T> CustomBox<T> {
  pub fn new(x: T) -> CustomBox<T> {
    CustomBox(x)
  }
}
