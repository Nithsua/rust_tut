/* Rust is a expressive. So, loops, if blocks, match blocks can all be intepreted 
as expressions
*/

fn main() {

    // The if else block is evaluated and based on the condition the block returns 
    // either the value from if or else
    let number = if true {
        45
    } else {
        65
    };
    println!("{}", number);

    // In rust a variable that's already declared can be declared again using the let keyword
    // it creates a new binding for the variable
    let mut number = number;

    // The loop is a unconditional loop which by default is a infinite loop. even the loop can be used
    // as a expression by using it with the break statement
    let value = loop {
        if number != 60 {
            number += 1;
        } else {
            break number;
        }
        println!("Infinite Loop");
    };

    println!("{}", value);


    // break with value only works with loop statement
    while number != 70 {
        number += 1;
    };
    println!("{}", value);

    // for loops are mostly used to iterate through a element in rust
    // for example consider 
    let arr = [23, 12, 65, 45, 90];
    let mut i = 0;

    while i < 5 {
        println!("{}", arr[i]);
        i += 1;
    }

    // Even tho above code works perfectly fine it's error prone and crashes the applcation
    // if given length is wrong and also the compile will add code to do check for the condition
    // during runtime since rust is efficiency consious the above can be done better using for
    for v in arr.iter() {
        println!("{}", v);
    }

    // Even for iterating through a range of value for can be used instead of while by using the 
    // Range type which is provided by the std library
    for i in 0..5 {
        println!("{}", i);
    }
}