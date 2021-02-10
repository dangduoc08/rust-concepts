use std::{fs::File, io::ErrorKind};

// Match Result<T, E>
// like try {
// ...do something
// } catch (err) {
// ...handle error
// }

pub fn use_result() {
    let res = File::open("hello.txt");

    let f = match res {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            test => {
                panic!("Problem opening the file: {:?}", test)
            }
        },
    };

    println!("{:?}", f);
}
