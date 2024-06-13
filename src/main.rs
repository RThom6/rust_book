// struct User {
    // active: bool,
    // username: String,
    // email: String,
    // age: u8,
// }

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

    _ownership();

    // Using a struct
    // let _user1 = User {
    //     active: true,
    //     username: String::from("Dave"),
    //     email: String::from("davemail@gmail.com"),
    //     age: 20,
    // }; // If using params, can use shorthand and not inclue "email:" just the var
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

    // For loop 12 -> 1: .rev() reverses the range, range not inclusive of 12?
    for number in (1..=12).rev() {
        println!("range: {number}");
    }

}

// '_' at start of identity means it is able to remain unused
fn _ownership() {
    let mut s = String::from("Hello"); // String type
    let mut _a = "test"; // &str type

    // _a.push_str(", world"); not possible as str not String
    s.push_str(", World!"); // Appends to the string

    let string = String::from("testing");
    println!("{string}");

    some_string(string);
    // After this due to move, if we tried printing string, it wouldnt compile

    // Cannot modify variable that is being borrowed by default, would need &mut Type instead
    let s = String::from("magic");
    
    some_reference(&s);
    println!("{s}");
    // Cannot have multiple references if a value has a mutable reference

    let slice = String::from("sliced");
    let slice = string_slicing(&slice);
    println!("{slice}");
}

// This function takes ownership of any input string
fn some_string(string: String) {
    println!("{string}");
}

// This function uses a reference instead of taking ownership
fn some_reference(s: &String) {
    println!("{s}");
}

// &str allows for String or str slices
fn string_slicing(s: &str) -> &str {
    println!("{s}");
    &s
}

// Defining a struct