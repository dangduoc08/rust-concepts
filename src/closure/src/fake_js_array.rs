// T: original vector item datatype
// F: closure as callback
// N: new vector item datatype

#[derive(Debug)]
pub struct ArrayPrototype<T> {
  pub vec: Vec<T>,
  pub len: usize,
}

pub trait ArrayMethod<T> {
  fn new(v: Vec<T>) -> ArrayPrototype<T>;
  fn map<F, N>(&self, closure: F) -> Vec<N>
  where
    F: Fn(&T, isize) -> N;
}

impl<T> ArrayMethod<T> for ArrayPrototype<T> {
  fn new(v: Vec<T>) -> ArrayPrototype<T> {
    let len: usize = v.len();

    ArrayPrototype { vec: v, len: len }
  }

  fn map<F, N>(&self, closure: F) -> Vec<N>
  where
    F: Fn(&T, isize) -> N,
  {
    let index: isize = -1;
    let mut new_vec: Vec<N> = Vec::<N>::new();
    for element in &self.vec {
      let new_element: N = closure(element, index);
      new_vec.push(new_element)
    }
    new_vec
  }
}
