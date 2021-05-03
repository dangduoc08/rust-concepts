use super::error_fnc;

pub fn fake() -> i8 {
    // Try
    // Cacth
    match error_fnc::err_fn(-10) {
        Ok(n) => n,
        Err(e) => panic!("{}", e), // Throw new Error
    }
}
