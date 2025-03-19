// Rust has signed (positive/negative) and unsigned (only positive) integer types of different sizes.
//
// i8, i16, i32, i64, i128:  Signed integers.
// u8, u16, u32, u64, u128:  Unigned integers.

fn main() {
    let x: i64 = -42;
    let y: u64 = 100;

    println!("Signed integer    :  {}", x);
    println!("Unsigned integer  :  {}", y);
}
