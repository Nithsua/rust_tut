pub fn run() {
    greeting("Hi", "Nitsua");

    let sum = add(5, 5);
    println!("{}", sum);

    //Closures
    let c: i32 = 45;
    let add_nums = |a: i32, b: i32| a + b + c;
    println!("{}", add_nums(6, 7));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
    //or
    //return a+b;
}
