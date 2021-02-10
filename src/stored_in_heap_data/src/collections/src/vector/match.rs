use super::type_assertion::create_type_assertion;

pub fn run_match() {
    let new_vec = create_type_assertion();

    let _second = new_vec[1];

    match new_vec.get(0) {
        Some(_second) => println!("Matched {}", _second),
        None => println!("Not matched"),
    }
}
