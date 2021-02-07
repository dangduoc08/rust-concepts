use super::type_assertion::create_type_assertion;

pub fn read_vector() {
    let new_vec = create_type_assertion();

    let elem_1: &i32 = &new_vec[0];
    let elem_2: &i32 = &new_vec[1];
    let elem_3: &i32 = &new_vec[2];

    // let elem_4: &i32 = &new_vec[3]; => Error: index out bound

    println!("Element index 0: {}", *elem_1);
    println!("Element index 1: {}", *elem_2);
    println!("Element index 2: {}", *elem_3);
}
