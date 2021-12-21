//OG Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// pub fn run() {
//     let mut c = Color {
//         red: 255,
//         green: 0,
//         blue: 0,
//     };

//     c.red = 200;
//     println!("Color: {} {} {}", c.red, c.green, c.blue);
// }

// Tuple Struct
// struct Color(u8, u8, u8);

// pub fn run() {
//     let mut c = Color(255, 0, 0);

//     println!("{} {} {}", c.0, c.1, c.2);
//     c.2 = 90;
//     println!("{} {} {}", c.0, c.1, c.2);
// }

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct a Person kinda like a Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut p = Person::new("Nitsua", "M");
    println!("{} {}", p.first_name, p.last_name);

    p.last_name = "G".to_string();
    println!("{}", p.full_name());

    p.set_last_name("M".to_string());
    println!("{}", p.full_name());

    println!("{:?}", p.name_to_tuple());
}
