enum Movement {
    Up, Down, Left,
    Right
}

fn move_avatar(movement: Movement) {
    // Perform action depending on info
    match movement {
        Movement::Up => println!("Acatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),

    }
}


pub fn run() {
    let avatar = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_avatar(avatar);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}