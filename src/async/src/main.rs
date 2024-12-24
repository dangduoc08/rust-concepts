use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    run_async();
    thread::sleep(Duration::from_millis(10000));

    Future
}

async fn run_async() -> String {
  println!("run");
  get_user().await;
  "connection from database".to_string()
}

async fn get_user() -> String {
  println!("get user");
  "this is user".to_string()
}