pub  fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[0]);

    // Re-assign value.
    numbers[2] = 20;

    //Add on to vector.
    numbers.push(5);
    numbers.push(6);

    //Pop off last value.

    numbers.pop();

    println!("Size of vector: {}", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
    for x in numbers.iter() {
        println!("Number: {}", x);

    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec multiplied: {:?}", numbers);

}
