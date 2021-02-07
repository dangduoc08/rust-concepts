use super::type_assertion::create_type_assertion;

pub fn get_vector() {
    let new_vec = create_type_assertion();

    let elem_1: Option<&i32> = new_vec.get(0);
    let elem_2: Option<&i32> = new_vec.get(1);
    let elem_3: Option<&i32> = new_vec.get(2);
    let elem_4: Option<&i32> = new_vec.get(3); // => Not error: return None

    println!("Element index 0: {:?}", elem_1);
    println!("Element index 1: {:?}", elem_2);
    println!("Element index 2: {:?}", elem_3);
    println!("Element index 3: {:?}", elem_4);
}
