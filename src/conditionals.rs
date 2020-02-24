//Conditionals - Used to check the condition of something.
pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knowns_person_of_age = true;
    //If/Else
    if age >= 21 && check_id || knowns_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave. ");

    } else {
        println!("Bartender: I'll need to see your ID");

    }

    let is_of_age = if age >= 21 {true} else { false };
    println!("Hello {}", is_of_age);
}