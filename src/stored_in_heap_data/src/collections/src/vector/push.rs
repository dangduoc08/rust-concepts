use super::type_annotation;

pub fn push() -> Vec<String> {
    let mut a_vec_string: Vec<String> = type_annotation::create_type_annotation();

    let val_1: String = String::from("Value 1");
    let val_2: String = String::from("Value 2");
    let val_3: String = String::from("Value 3");

    a_vec_string.push(val_1);
    a_vec_string.push(val_2);
    a_vec_string.push(val_3);

    a_vec_string
}
