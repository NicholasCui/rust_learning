fn main() {
    // Scalar Types
    let int_type: u32 = "42".parse().expect("Not a number");
    println!("int: {}", int_type);

    let float_type: f32 = 3.2;
    println!("float: {}", float_type);

    let bool_type: bool = false;
    println!("bool: {}", bool_type);

    let char_type: char = 'Z';
    println!("char: {}", char_type);

    // Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple: {}", tup.0);

    let arr = [1, 2, 3, 4, 5];
    println!("arr: {}", arr[0]);
}