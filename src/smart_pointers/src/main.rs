/*
  Recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
  - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
  - Box<T> allows immutable or mutable borrows checked at compile time;
  - Rc<T> allows only immutable borrows checked at compile time;
  - RefCell<T> allows immutable or mutable borrows checked at runtime.
  - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.


*/

use smart_pointers::{
  box_linked_list::BoxLinkedList::{Nil as BoxNil, Node as BoxNode},
  custom_box,
  rc_linked_list::RCLinkedList::{Nil as RCNil, Node as RCNode},
  standard_box,
};
use std::rc::Rc;

fn main() {
  let std_box = standard_box::new(10);
  println!("{:?}", std_box);

  let cus_box = custom_box::CustomBox::new(10);
  println!("{:?}", *cus_box);

  // can dealloc by drop();

  let box_linked_list = BoxNode(
    0,
    Box::new(BoxNode(1, Box::new(BoxNode(2, Box::new(BoxNil))))),
  );
  println!("{:?}", box_linked_list);

  let box_linked_list_root_1 = BoxNode(-1, Box::new(box_linked_list));
  println!("{:?}", box_linked_list_root_1);

  // error: due to box_linked_list was moved
  let box_linked_list_root_2 = BoxNode(-2, Box::new(box_linked_list));
  println!("{:?}", box_linked_list_root_2);

  let rc_linked_list_2 = Rc::new(RCNode(2, Rc::new(RCNil)));
  println!("{:?}", rc_linked_list_2);

  let rc_linked_list_1 = RCNode(1, Rc::clone(&rc_linked_list_2));
  println!("{:?}", rc_linked_list_1);

  let rc_linked_list_3 = RCNode(3, Rc::clone(&rc_linked_list_2));
  println!("{:?}", rc_linked_list_3);

  {
    let rc_linked_list_4 = RCNode(4, Rc::clone(&rc_linked_list_2));
    println!("{:?}", rc_linked_list_4);
  }

  println!("{}", Rc::strong_count(&rc_linked_list_2));
}
