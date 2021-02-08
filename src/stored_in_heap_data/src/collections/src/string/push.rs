use super::instance::create_string;

pub fn push_string() -> String {
    let mut s1: String = create_string("New  String");
    s1.push_str("pushed string");
    s1
}
