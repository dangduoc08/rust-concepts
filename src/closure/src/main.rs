use closure::fake_js_array::{ArrayMethod, ArrayPrototype};

fn main() {
  let new_arr: Vec<usize> = vec![0, 2, 4, 6, 9, 10, 11];

  let arr_prototype: ArrayPrototype<usize> = ArrayPrototype::<usize>::new(new_arr);

  let mapped_arr: Vec<String> = arr_prototype.map(|element: &usize, _index: isize| -> String {
    let mut result: usize = *element;
    if element % 2 == 0 {
      result = *element * 2;
    }

    result.to_string()
  });

  println!("arr_prototype: {:#?}", arr_prototype.vec);
  println!("mapped_arr: {:#?}", mapped_arr);
}
