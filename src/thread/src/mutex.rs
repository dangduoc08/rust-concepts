use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
  let num = Arc::new(Mutex::new(10));
  let cloned_num = Arc::clone(&num);
  println!("Num before lock {:?}", num);
  let join_handler = thread::spawn(move || {
    let mut i = cloned_num.lock().unwrap();
    *i += 100;
  });

  join_handler.join().unwrap();
  println!("Num after lock {:?}", num);
}
