pub fn run() {
    let mut vector: Vec<i32> = vec![334, 56];

    println!("{:?} {} {}", vector, vector.capacity(), vector.len());

    vector.insert(vector.len(), 45);

    println!("{:?} {} {}", vector, vector.capacity(), vector.len());

    vector.push(76);

    println!("{:?} {} {}", vector, vector.capacity(), vector.len());

    vector.insert(2, 2342);

    println!("{:?} {} {}", vector, vector.capacity(), vector.len());

    for x in vector.iter() {
        println!("{}", x);
        // *x = *x * 2;
    }

    for element in vector.iter_mut() {
        // println!("{}", element);
        *element = *element * 2;
    }

    println!("{:?}", vector);
}
