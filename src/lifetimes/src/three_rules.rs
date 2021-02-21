pub trait First {
    // Each parameter that is a reference gets its own lifetime parameter.
    // In other words, a function with one parameter gets one lifetime parameter
    fn bar<'a>(x: &'a i32);

    // Function with two parameters gets two separate lifetime parameters:
    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i16;
}

pub trait Second {
    // The second rule is if there is exactly one input lifetime parameter,
    // that lifetime is assigned to all output lifetime parameters
    fn baz<'a>(x: &'a i32) -> &'a i32;
}

pub trait Third {
    // The third rule is if there are multiple input lifetime parameters,
    // but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters.
    // This third rule makes methods much nicer to read and write because fewer symbols are necessary.
    fn qux(&self, x: &i32, y: &i8) -> &i16; // No need lifetimes
}
