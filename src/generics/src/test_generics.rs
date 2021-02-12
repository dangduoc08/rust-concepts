use super::types;

pub fn test() {
    let p = types::Point::<u8, i8, f32> {
        x: 10,
        y: -10,
        z: 10.10,
    };

    println!("{:#?}", p);

    println!("{:?}", p.draw());

    let _success: types::Status<u8> = types::Status::Success(0);
    let _error: types::Status<u8> = types::Status::Error(1);
    let _cancel: types::Status<u8> = types::Status::Cancel(2);

    match _cancel {
        types::Status::Success(s) => println!("success {}", s),
        other_val_1 => match other_val_1 {
            types::Status::Error(e) => println!("error {}", e),
            other_val_2 => match other_val_2 {
                types::Status::Cancel(c) => println!("cancel {}", c),
                _ => (),
            },
        },
    }
}
