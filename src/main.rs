fn main() {
    // Data Types:

    // Integer -(2^(n-1)) -> 2^(n-1) - 1
    // Syntax: i8; i16; i32; i64; i128
    // isize (depend on the kind of computer your program is running)
    let int: i32 = -10;

    // Unsigned integer 0 -> 2^(n-1)
    // Syntax: u8; u16; u32; u64; u128
    // usize (depend on the kind of computer your program is running)
    let unint: u32 = 20;

    // Floating-point
    // Syntax: f32; f64
    let float: f32 = 18.13;

    // Boolean
    // Syntax: bool
    let t: bool = true;

    // Char = 4 bytes, use single quotes
    // Syntax: char
    let ch: char = 'q';

    // String
    // let s = "This is string";

    // Array
    // Syntax: [<Data types>; <Capacity>]
    let arr: [u8; 3] = [1, 2, 3];

    // Tuple
    // Syntax: (<Data types>)
    let tup: (i8, u32, char) = (-15, 29, 'c');

    println!("This is integer: {}", int);
    println!("This is unsigned integer: {}", unint);
    println!("This is float: {}", float);
    println!("This is boolean: {}", t);
    println!("This is character: {}", ch);
    println!("This is tuple: {}", tup.0);
    println!("This is array: {}", arr[0]);

    plus(-10, -2);

    let result_from_minus = minus(-20, -100);
    println!(
        "This is the result when invoke minus function: {}",
        result_from_minus
    );
}

fn plus(a: i8, b: i8) {
    print!("The result when {} + {} = {} \n", a, b, a + b)
}

// We don’t name return values, but we do declare their type after an arrow (->).
// In Rust, the return value of the function is synonymous
// with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return
fn minus(a: i32, b: i32) -> i32 {
    // return a - b;
    // or
    a - b
    // diff at ;
}
