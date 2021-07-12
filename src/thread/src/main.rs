use thread::{dead_lock, message_passing, mutex, spawn};

fn main() {
  spawn::run();
  mutex::run();
  message_passing::run();
  dead_lock::run();
}
