//A tuple is a collection of elements of different data type just like in python
//but can have a maximum of only 12 elements

pub fn run() {
    let person = ("John", "Doe", 34);
    println!("{:?}", person);

    println!("{} {} {}", person.0, person.1, person.2);
}
