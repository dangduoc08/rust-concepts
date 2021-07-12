use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Deadlock occurs when:
// - Program runs more than 1 thread
// - Program has more than 1 lock
pub fn run() {
  println!("DEAD LOCK");
  let total_worker: i16 = 2;
  let mut join_handlers: Vec<thread::JoinHandle<()>> = Vec::new();

  let resource_a: Arc<Mutex<i8>> = Arc::new(Mutex::new(0));
  let resource_b: Arc<Mutex<i8>> = Arc::new(Mutex::new(0));

  for i in 0..total_worker {
    let is_thread_1: bool = i == 0;
    let resource_a_clone = Arc::clone(&resource_a);
    let resource_b_clone = Arc::clone(&resource_b);

    let join_handle = thread::spawn(move || {
      if is_thread_1 {
        let mut num_a = resource_a_clone.lock().unwrap();
        *num_a += 10;
        println!("Thread 1 is locking resource_a");
        println!("Thread 1 needs resource_b");
        thread::sleep(Duration::from_secs(1));
        let mut num_b = resource_b_clone.lock().unwrap();
        println!("Thread 1 is locking resource_b");
        *num_b += 10;
        println!("Thread 1 unlocks resource_b");
      } else {
        let mut num_b = resource_b_clone.lock().unwrap();
        println!("Thread 2 is locking resource_b");
        println!("Thread 2 needs resource_a");
        *num_b += 10;
        thread::sleep(Duration::from_secs(1));
        let mut num_a = resource_a_clone.lock().unwrap();
        println!("Thread 2 is locking resource_a");
        *num_a += 10;
        println!("Thread 2 unlocks resource_a");
      }
    });

    join_handlers.push(join_handle);
  }

  for join_handle in join_handlers {
    join_handle.join().unwrap();
  }

  println!("If you see this, it's mean there is no deadlock occurs");
  println!("resource_a = {:?}", resource_a);
  println!("resource_b = {:?}", resource_b);
}
