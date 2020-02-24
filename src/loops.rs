pub fn run() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Numbers: {}", count);
        if count == 20 {
            break;
        }
    }
    count = 1;
    while count <= 100 {
        if count  % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz")
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }
}