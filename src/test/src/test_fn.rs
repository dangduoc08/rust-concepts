pub fn add(a: i128, b: i128) -> Result<i128, String> {
    if a < 0 || b < 0 {
        let err_msg = "a and b must be greater than 0";
        Err(err_msg.to_string())
    } else {
        Ok(a + b)
    }
}

pub fn panic_add(a: i128, b: i128) -> i128 {
    let value = match add(a, b) {
        Ok(_result) => _result,
        Err(err_msg) => panic!(err_msg),
    };

    value
}
