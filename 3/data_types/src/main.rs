fn main() {
    let int_type: u32 = "42".parse().expect("Not a number");
    println!("int: {}", int_type);

    let float_type: f32 = 3.2;
    println!("float: {}", float_type);
}