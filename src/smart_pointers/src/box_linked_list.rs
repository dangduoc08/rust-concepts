#[derive(Debug)]
pub enum BoxLinkedList<T> {
  Node(T, Box<BoxLinkedList<T>>),
  Nil,
}
