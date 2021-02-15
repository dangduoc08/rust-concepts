use super::traits::{Asian, Human};

pub fn notify1(a_person: &impl Human) {
    println!("{}", a_person.greet())
}

// OR

// Use when want to constrains parameter types
// This is call trait bound
pub fn notify2<T: Human>(a_person: &T) {
    println!("{}", a_person.greet())
}

// OR

pub fn notify3<T>(a_person: &T)
where
    T: Human + Asian,
{
    println!("{}", a_person.greet())
}
