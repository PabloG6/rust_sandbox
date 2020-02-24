pub fn run() {
    let name = "Brad";

    //mut makes the variable mutable.
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);

    //Define constant;
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
    
}