use super::type_assertion::create_type_assertion;

pub fn iter() {
    let mut new_vec = create_type_assertion();

    for elem in &mut new_vec {
        *elem += 10;
        println!("{}", elem);
    }
}
