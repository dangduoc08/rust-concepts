use std::rc::Rc;

#[derive(Debug)]
pub enum RCLinkedList<T> {
  Node(T, Rc<RCLinkedList<T>>),
  Nil,
}
