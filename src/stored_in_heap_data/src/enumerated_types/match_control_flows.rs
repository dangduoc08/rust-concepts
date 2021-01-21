pub fn test_match(a_number: i8) -> String {
    match a_number {
        0 => String::from("Match with 0"),
        10 => String::from("Match with 10"),
        100 => String::from("Match with 100"),
        _ => String::from("Unmatch"),
    }
}

pub fn match_with_some(a_some_value: Option<isize>) -> Option<isize> {
    match a_some_value {
        Some(10) => Some(10),
        Some(i) => Some(i * 10),
        None => None,
    }
}

pub fn handle_product_variation_status(
    product_variation: super::definition::ProductVariation,
) -> super::definition::ProductVariation {
    match product_variation {
        super::definition::ProductVariation::SyncPrice(state) => {
            super::definition::ProductVariation::SyncPrice(state)
        }
        _ => product_variation,
    }
}
