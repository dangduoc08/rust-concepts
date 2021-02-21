// One special lifetime we need to discuss is 'static,
// which means that this reference can live for the entire duration of the program.
// All string literals have the 'static lifetime,
pub fn test_static_lifetimes() {
    let _s: &'static str = "I have a static lifetime.";
}
