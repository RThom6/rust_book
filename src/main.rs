fn main() {
    println!("Hello, world!");

    // Constant - uppercase with underscores
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1; // still not mutable but by using let, it shadows the first definition of x

    {
        let x = x * 2;
        println!("{x}"); // x shadows the other x only within this scope
    }

    println!("{x}");

    data_types(5);
}

fn data_types(_value: i32) {

    // unsigned 32-bit int
    let guess: u32 = 16;

    println!("{guess}");

    /*
    *   Integer types
    *   i&u 8, 16, 32, 64, 128
    */

    // Tuple
    // Each position can have a type
    let tuple: (i32, f64, u8) = (-500, 6.4, 1);
    // Can deconstruct tuple with pattern matching
    let (x, y, z) = tuple;
    // Or
    let value = tuple.1;

    println!("{x}, {y}, {z}, {value}");

    // Use arrays when you have a set number of elements e.g. months of the year
    // Otherwise use vectors 
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Function implicitly returns last expression, or can use 'return x;'

    // For loop 12 -> 1: .rev() reverses the range
    for number in (1..12).rev() {
        println!("{number}");
    }

}
