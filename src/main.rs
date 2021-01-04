fn main() {
    println!(
        "Blue violet: {:?}",
        stored_in_heap_data::struct_types::instance::BLUE_VIOLET
    );
    println!(
        "White: {:?}",
        stored_in_heap_data::struct_types::instance::WHITE
    );
    println!(
        "A guy: {:#?}",
        stored_in_heap_data::struct_types::instance::create_person(
            String::from("John"),
            String::from("Doe"),
            26,
            String::from("50/45 Nhat Chi Mai st, ward 13, Tan Binh dist, Ho Chi Minh city")
        )
    );

    let another_guy = stored_in_heap_data::struct_types::definition::definition::Person {
        first_name: String::from("John"),
        last_name: String::from("Cena"),
        age: 45,
        address: String::from("Canifornia"),
    };

    another_guy.greet();
    another_guy.run();
    another_guy.laugh();
}
