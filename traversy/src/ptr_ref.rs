pub fn run() {
    let mut arr1 = [1, 2, 3, 4];
    let arr2 = arr1;
    println!("{:?} {:?}", arr1, arr2);
    arr1[3] = 45;
    println!("{:?} {:?}", arr1, arr2);

    let mut vector1 = vec![23, 43];
    let vector2 = &vector1;

    println!("{:?} {:?}", vector1, vector2);

    vector1.push(45);
    // println!("{:?} {:?}", vector1, vector2);
}
