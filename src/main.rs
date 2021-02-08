use stored_in_heap_data::collections::string::{concatenation, instance, push, slice};

fn main() {
    println!("{}", instance::create_string("Test string"));
    println!("{}", push::push_string());
    format!("{}", concatenation::concate());
    slice::test_slice();
}
