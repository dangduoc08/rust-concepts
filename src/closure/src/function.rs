/* A Javascript closure
function parent() {
 let x = 10

  function child() {

    // x still reference to lexical environment
    return x + 10
  }

  return child
}
*/

/* Invalid Rust closure
fn parent() -> fn() -> isize {
  let x: isize = 10;

  fn child() -> isize {

    // Unlike Javascript, Rust can't capture dynamic environment in a fn item
    // use closure instead
    x + 10
  }

  child
}
*/

/* Closure syntax
  fn function_add_1(n: i8) -> i8 { n + 1 };
  let closure_add_1 = |n: i8| -> i8 { n + 1 };
  let closure_add_2 = |n| { n + 1 };
  let closure_add_3 = |n| n + 1;
*/

// Valid Rust closure
pub fn parent() -> impl Fn() -> isize {
  let x: isize = 10;

  // x still reference to lexical environment
  let child = move || -> isize { x + 10 };

  child
}
