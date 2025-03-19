

fn main() {
    // Rust has signed (positive/negative) and unsigned (only positive) integer types of different sizes.
    // i8, i16, i32, i64, i128:  Signed integers.
    // u8, u16, u32, u64, u128:  Unigned integers.
    let x: i64 = -42;
    let y: u64 = 100;

    println!("Signed integer   : {}", x);
    println!("Unsigned integer : {}", y);

    // Float - f32, f64
    let pi: f64 = 3.1415926535897932384626433832795;
    println!("Pi               : {}", pi);

    // Bool (boolean) - true, false
    let is_running: bool = true;
    println!("Is running       : {}", is_running);

    // Char (character) - char
    let letter: char = 'A';
    println!("Letter           : {}", letter);
}
