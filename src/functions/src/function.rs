
// Function not return value
fn plus(a: i8, b: i8) {
  print!("The result when {} + {} = {} \n", a, b, a + b)
}

// We don’t name return values, but we do declare their type after an arrow (->).
// In Rust, the return value of the function is synonymous
// with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return
fn minus(a: i32, b: i32) -> i32 {
  // return a - b;
  // or
  a - b
  // diff at ;
}

// Control flows
fn pt2(a: f64, b: f64, c: f64) {
  let delta: f64 = b.powi(2) - (4.0 * a * c);

  if delta < 0.0 {
      println!("Equation has no result");
  } else if delta == 0.0 {
      let x: f64 = -b / (2.0 * a);
      println!("Equation has double results x1=x2 {}", x)
  } else {
      let x1: f64 = (-b + delta.sqrt()) / (2.0 * a);
      let x2: f64 = (-b - delta.sqrt()) / (2.0 * a);
      println!("Equation has 2 diff results x1={}, x2={}", x1, x2);
  }
}

// Loop with break
fn infinity_loop() -> i8 {
  let mut i: i8 = 1;
  loop {
      i += i;
      println!("{}", i);
      if i > 32 {
          break i + 1;
      }
  }
}

// While
fn infinity_while() -> i8 {
  let mut i: i8 = 1;
  while i <= 32 {
      i += i;
      println!("{}", i);
  }
  i + 1
}

// For array
fn for_each() {
  let arr: [isize; 4] = [1, 2, 3, 4];
  let mut index: i8 = 0;
  for element in arr.iter() {
      println!("Element at index {} has value: {}", index, element);
      index += 1;
  }
}