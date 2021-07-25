// Unsafe abilities:
// - Dereference a raw pointer
// - Call unsafe functions or methods
// - Creating a Safe Abstraction over Unsafe Code, dangerous fn has an usafe code, but no need unsafe block to invoke its
// - Using extern Functions to Call External Code from another languages
// - Accessing or Modifying a Mutable Static Variable
// - Implementing an Unsafe Trait
// - Accessing Fields of a Union

extern "C" {
  pub fn abs(input: i32) -> i32;
}

// Global variables
pub static mut NODE_ENV: &str = "development";

// Change global variables can cause data race
// therefore its must be wrapped in unsafe block

pub fn change_glob_var() {
  unsafe {
    NODE_ENV = "production";
  }
}

unsafe trait Foo {
  // methods go here
}

unsafe impl Foo for i32 {
  // method implementations go here
}

fn dangerous() {
  let mut c: i32 = 10;

  let raw_pointer = &mut c as *mut i32;

  unsafe { *raw_pointer = 100 }
}

pub fn run() {
  let mut a = 10;

  let raw1 = &a as *const i32;
  let raw2 = &mut a as *mut i32;

  unsafe {
    // Deref raw pointer
    *raw2 = 101;
    println!("{:?}", raw1);
    println!("{:?}", raw2);
  };

  dangerous();

  let address = 0x01234usize;
  let _r = address as *mut i32;
}
