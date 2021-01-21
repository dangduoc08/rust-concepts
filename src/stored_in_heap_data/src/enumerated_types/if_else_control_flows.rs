pub fn handle_match_enum(num: isize) {
    let some_u8_value = Some(num);
    match some_u8_value {
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        _ => (),
    }
}

// Same behavior but if statement is shorter

pub fn handle_if_enum(num: isize) {
    let some_u8_value = Some(num);
    if let Some(3) = some_u8_value {
        println!("three");
    } else if let Some(4) = some_u8_value {
        println!("four")
    }
}
