//Primitiver str = Immutable fixed-length string somewhere in memory
//String = Growable, heap allocated data structure - Use when you need to modify or own string data.

pub fn run() {

    //immutable string.
    let _hello_immutable = "Hello";

    //mutable string. immutable if the mut prefix isn't used. 
    let mut hello_mutable = String::from("Hello ");
    println!("Length: {}", hello_mutable.len());

    hello_mutable.push('W');
    hello_mutable.push_str("world");


    //Capacity in bytes
    println!("Capacity: {}", hello_mutable.capacity());

    //check if empty
    println!("Is Empty: {}", hello_mutable.is_empty());

    //contains
    println!("Contains 'World' {}", hello_mutable.contains("World"));

    // Replace
    println!("Replace: {}", hello_mutable.replace("World", "There"));

    for word in hello_mutable.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);
    println!("{}", hello_mutable);

}