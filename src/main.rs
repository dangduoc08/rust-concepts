fn main() {
  // Deadlock

  // let resource_a = Arc::new(Mutex::new(0));
  // let resource_b = Arc::new(Mutex::new(0));
  // let mut join_handlers = vec![];

  // let clone_resource_a_1 = Arc::clone(&resource_a);
  // let clone_resource_b_1 = Arc::clone(&resource_b);
  // let handler_1 = thread::spawn(move || {
  //   let mut a = clone_resource_a_1.lock().unwrap();
  //   println!("a is locking");
  //   *a += 1;
  //   let mut b = clone_resource_b_1.lock().unwrap();
  //   println!("b is locking");
  //   *b += 1;
  // });
  // join_handlers.push(handler_1);

  // let clone_resource_a_2 = Arc::clone(&resource_a);
  // let clone_resource_b_2 = Arc::clone(&resource_b);
  // let handler_2 = thread::spawn(move || {
  //   let mut b = clone_resource_b_2.lock().unwrap();
  //   println!("b is locking");
  //   *b += 1;
  //   let mut a = clone_resource_a_2.lock().unwrap();
  //   println!("a is locking");
  //   *a += 1;
  // });
  // join_handlers.push(handler_2);

  // for join_handler in join_handlers {
  //   join_handler.join().unwrap();
  // }

  // println!("resource_a: {:?}", resource_a);
  // println!("resource_b: {:?}", resource_b);

  println!("Hello world");

}
