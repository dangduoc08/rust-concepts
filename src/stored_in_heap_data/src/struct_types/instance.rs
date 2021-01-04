use super::definition::definition::{Person, RGB};

pub fn create_person(first_name: String, last_name: String, age: i8, address: String) -> Person {
    Person {
        first_name,
        last_name,
        age,
        address,
    }
}

pub const BLUE_VIOLET: RGB = RGB(138, 43, 226);
pub const WHITE: RGB = RGB(255, 255, 255);
