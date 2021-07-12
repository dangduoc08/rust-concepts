use std::thread;
use std::time::Duration;

pub fn run() {
  thread::spawn(|| {
    for i in 0..10 {
      println!("Spawned thread: {}", i);
      thread::sleep(Duration::from_millis(500));
    }
  });

  for i in 0..10 {
    println!("Main thread: {}", i);
    thread::sleep(Duration::from_millis(250));
  }
}
