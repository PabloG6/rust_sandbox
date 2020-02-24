/*
Primitive types -- 
Integers: u8, i8, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//Rust is statically typed, so it much know the types of all variables at compile time
//however the compiler can usually infer what type we want to use based on the value 
//and how we use it.

pub fn run() {

    //Default is i32.
    let x = 1;

    // Default is f64.
    let y = 2.5;

    let z: i64 = 48348383884549;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));


}