mod print;
mod variables;
mod types;
mod string;
mod arrays;
mod pointer_ref;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod structs;
mod enums;
fn main() {
    title("Start of tutorial!");
    title("1. Print");
    print::run();

    title("2. Variables");
    variables::run();

    title("3. Types");
    types::run();
    title("4. String");
    string::run();

    title("5. Arrays", );
    arrays::run();

    title("5. Vectors");
    vectors::run();

    title("6. Conditionals");
    conditionals::run();

    title("7. Loops");
    loops::run();

    title("8. Functions");
    functions::run();

    title("9. Pointer Ref");
    pointer_ref::run();

    title("10. Struct");
    structs::run();

    title("11. Enums");
    enums::run();



}



fn title(title: &str) {
    println!("<<---------{}-------->", title);
}