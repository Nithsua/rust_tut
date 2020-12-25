pub fn run() {
    //Immutable
    let hello = "Hello";

    //Mutable Heap based data structure that is growable and dynamic
    let mut hello_heap = String::from("Hello ");

    println!("{} {}", hello, hello_heap);

    println!("{}", hello.len());

    println!("{} {}", hello_heap.len(), hello_heap.capacity());

    println!("{}", hello.is_empty());
    println!("{}", hello_heap.is_empty());

    hello_heap.push('W');

    hello_heap.push_str("orld!!!");

    println!("{}", hello_heap);

    for character in hello_heap.split_whitespace() {
        println!("{}", character);
    }

    for character in hello_heap.splitn(hello_heap.len(), "") {
        println!("{}", character);
    }

    let mut s = String::with_capacity(10);
    s.push('W');
    s.push('H');

    //Assertion like in Dart
    assert_eq!(3, s.len());

    println!("{}", s);
}
