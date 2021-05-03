// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

// --test <test_file_name> // To run all the tests in a particular integration test file

use testing::test_fn;

mod integration_div;

#[test]
fn it_should_be_add_i128() {
  let a: i128 = test_fn::create_variable::<i128>(10);
  let b: i128 = test_fn::create_variable::<i128>(10);

  let sum_result = match test_fn::sum(a, b) {
    Ok(res) => res,
    Err(err_msg) => panic!("{}", err_msg),
  };

  assert_eq!(sum_result, 20, "Error message: expect {} + {} = 20", a, b);
}

#[test]
fn it_should_be_div_i128() {
  assert_eq!(integration_div::get_div_result(), 9);
}
