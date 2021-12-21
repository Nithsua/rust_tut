pub fn run() {
    let mut name_string: String;
    let mut name_str: &str;
    let mut age;

    name_string = String::from("Doople Doople");
    age = 38;
    println!("{} {}", name_string.capacity(), name_string.len());

    println!("My name is {} and I am {}", name_string, age);

    name_string.push_str("fu");
    age = 90;
    println!("{} {}", name_string.capacity(), name_string.len());

    println!("My name is {} and I am {}", name_string, age);

    name_str = "Loser";
    age = 123;
    println!("{}", name_str.len());

    println!("My name is {} and I am {}", name_string, age);

    name_str = "Los";
    age = 64;
    println!("{}", name_str.len());

    println!("My name is {} and I am {}", name_string, age);

    let (name, age) = ("John", 34);
    println!("{} {}", name, age);
}
