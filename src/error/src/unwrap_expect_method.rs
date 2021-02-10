use super::error_fnc::err_fn;

pub fn unwrap(n: i8) {
    err_fn(n).unwrap();
}

pub fn expect(n: i8, msg: &str) {
    err_fn(n).expect(msg);
}
