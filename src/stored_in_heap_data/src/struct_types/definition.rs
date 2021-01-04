pub mod definition {

    // Normal struct
    #[derive(Debug)]
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
        pub age: i8,
        pub address: String,
    }

    impl self::Person {
        pub fn greet(&self) {
            println!("Hello")
        }
    }

    // Tuple struct
    #[derive(Debug)]
    pub struct RGB(pub i16, pub i16, pub i16);
}

impl definition::Person {
    pub fn run(&self) {
        println!("Go go")
    }
}
