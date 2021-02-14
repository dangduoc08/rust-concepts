pub mod Pointer {
    const s1: String = String::from("hihi");
    const s2: &String = &s1;
    const s3: *const String = &s1;
    println!("{:?}", s3);
    println!("{:?}", &s1 as *const String);
    println!("{:?}", s2 as *const String);
}
