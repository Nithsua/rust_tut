pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!(
        "{} {} {} {} {}",
        numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]
    );

    numbers[2] = 45;

    println!("{:?}", numbers);

    println!("{}", numbers.len());

    //Arrays are stack allocated
    println!("Array uses {} bytes", std::mem::size_of_val(&numbers));

    //Get slice

    let slice: &[i32] = &numbers[0..3];

    println!("{:?}", slice);
}
