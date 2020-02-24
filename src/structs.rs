//Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTuple(u8, u8, u8);
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
    Person {
        first_name: first.to_string(),
        last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();

    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };


    println!("Before Color: {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("After Color: {} {} {}", c.red, c.green, c.blue);

    let c_tuple = ColorTuple(255, 29, 49);
    println!("Color Tuple: {} {} {}", c_tuple.0, c_tuple.1, c_tuple.2);

    let mut p = Person::new("John", "Doe");
    
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person Married {}", p.full_name());
    println!("Person Tuple{:?}", p.to_tuple());
}

