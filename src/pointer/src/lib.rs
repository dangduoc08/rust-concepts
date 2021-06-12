// Memory
// Ownership
// Borrowing

/*
  Stack
  - Extremely Fast
  - Values must have Fixed Sizes
  - Always puts data in on Top
  - data pushed down as new data comes in
*/

/*
  Heap
  - Less Organized and Slower
  - Accepts Dynamicly Sized Data or Data that can Grow
  - Returns a Pointer which goes on the stack
  - Pointer points to where the data is on the Heap
*/

/*
  Ownership Rules:
    1) Each value has a variable which is its owner.
    2) There can only be one owner at any given time.
    3) When the owner goes out of scope, the value will be dropped out of memory.
*/

/*
  Borrowing Rules:
    1) Allowed infinite borrows for readonly access.
    2) Readonly borrows make the original data immutable for their duration.
    3) Only allowed to pass one borrow at a time for write access/mutability. // Avoid data racing
*/

/* Rust Stack | Copy Types
    bool
    character
    numbers
    slices
    fix sized arrays
    tuples containing primitives
    function pointers
*/

pub mod pointer {
  pub fn r#ref() {
    // Stack
    println!("--- Stack ---");
    let mut a: isize = 10;
    println!("mut a = {}", a);
    println!("memory address's &a = {:?}", &a as *const isize);
    println!("mut a owns {}", a);
    let b: isize = a;
    println!("b = a");
    println!("b = {}", b);
    println!("memory address's &b = {:?}", &b as *const isize);
    println!("b don't borrowed a");
    println!("b coppies value's a in stack");
    a = 20;
    println!("mut a = {}", a);
    println!("mut a reassign {} freely", a);
    println!("b still = {}", b);
    println!("-------------");

    let c: isize = 10;
    println!("mut c = {}", c);
    println!("memory address's &c = {:?}", &c as *const isize);
    println!("mut c owns {}", c);
    let d: &isize = &c;
    println!("d = &c");
    println!("d references c");
    println!("d = {}", d);
    println!("reference &d = {:?}", d as *const isize);
    println!("memory address's &d = {:?}", &d as *const &isize);
    println!("d borrowed c");
    println!(
      "mut c = {} => error => because it's was borrowed by d, and d is immutable",
      20
    );
    println!("mut c cannot reassign to {}", 20);

    // Heap
    println!("--- Heap ---");
    #[derive(Debug)]
    enum Status {
      Failed(isize),
      Successed(isize),
    }

    let mut d: Status = Status::Successed(1);
    println!("mut d = {:?}", d);
    println!("memory address's &d = {:?}", &d as *const Status);
    println!("mut d owns {:?}", d);
    let i: Status = d;
    println!("i = d");
    println!("i = {:?}", i);
    println!("memory address's &i = {:?}", &i as *const Status);
    println!("d was moved");
    println!(
      "println!(`{:?}`, d); => error => because c take ownership's d",
      ":?"
    );
    println!("d moved, d is no longer use");
    d = Status::Failed(0);
    println!("mut d = {:?}", d);
    println!("memory address's &d = {:?}", &d as *const Status);
    println!("mut d owns {:?} again", d);

    // Pointer
    println!("--- Pointer ---");
    let i: i8 = 10;
    println!("i = {}", i);
    println!("memory address's i = {:?}", &i as *const i8);
    let f: &i8 = &i;
    println!("f references i");
    println!("&f = {}", f);
    println!("raw pointer's f = {:?}", f as *const i8);
    println!("memory address's f = {:?}", &f as *const &i8);
    let g = &i as *const i8;
    println!("g point to i");
    println!("value's g = {:?}", g);
    println!("dereference g = {}", unsafe { *g });
  }
}
