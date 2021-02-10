pub fn err_fn(n: i8) -> Result<i8, String> {
    if n >= 0 {
        return Ok(n);
    }
    Err(String::from("Only accept positive integer"))
}
