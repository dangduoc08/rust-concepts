use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Like channel in Golang
pub fn run() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    for i in 0..6 {
      tx.send(i).unwrap();
      println!("Sent: {}", i);
      thread::sleep(Duration::from_secs(1))
    }
  });

  for receiver in rx {
    println!("Received: {}", receiver);
  }

  println!("End main");
}
