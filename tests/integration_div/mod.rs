use testing::test_fn::{create_variable, div};

pub fn get_div_result() -> i128 {
  let a: i128 = create_variable::<i128>(27);
  let b: i128 = create_variable::<i128>(3);
  match div(a, b) {
    Ok(res) => res,
    Err(err_msg) => panic!("{}", err_msg),
  }
}
