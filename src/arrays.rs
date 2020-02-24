//Arrays - Fixed list where elements 
pub  fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[0]);

    numbers[2] = 20;

    println!("Array Length: {}", numbers.len());
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    let slice:&[i32] = &numbers[1..3];
    println!("{:?}", slice);

}