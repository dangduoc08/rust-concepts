pub mod pointer {
    pub fn r#ref() {
        let s1: String = String::from("hihi");
        let s2: &String = &s1;
        let s3: *const String = &s1;
        println!("{:?}", s3);
        println!("{:?}", &s1 as *const String);
        println!("{:?}", s2 as *const String);
    }
}
