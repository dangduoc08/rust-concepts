use super::error_fnc;

fn test_question_mark_operation(n: i8) -> Result<i8, Option<String>> {
    let res: i8 = error_fnc::err_fn(n)?; // => Work like match but shorter

    Ok(res * 10)
}

pub fn using() {
    let number: i8 = -10;
    match test_question_mark_operation(number) {
        Ok(n) => println!("Number will be multiply 10 = {}", n),
        Err(err) => match err {
            Some(s1) => println!("This is s1 = {}", s1),
            None => panic!("Undefined error"),
        },
    }
}
