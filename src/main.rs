use std::io;

// latest version

#[allow(unused)]
fn main() {

    /*
    slice_type();
    ownership();
    for_loop();
    counting();
    counting_old_way();
    another_function(5);
    variables();

    // expression does not have semicolon at the end
    // expressions evaluate to a value
    // statement has a semicolon at the end  and no return value
    let _y = {
        let x = 3;
        x + 1
    };
    let f = five();
    println!("five return: {f}");
    let p1 = plus_one(f);
    println!("five return: {p1}");
    control_flow();
    */
}
// 

// 5. Using structs to structure related data

// field init shorthand
fn build_user_field_init(email: String, username: String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count:1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email:email,
        sign_in_count: 1,
    } // ide nem kell ; mert ekkor implicite visszaadja
      // ha lenne ; akkor returnozni kellen egy változón keresztül
}

fn create_struct(){
    let mut user1 = User{
        is_active:true,
        username: String::from("janouser11"),
        email: String::from("jano@nuc.hu"),
        sign_in_count: 3,
    }
    user1.email = String::from("jano@dapc.hu");
    let user2 = build_user(
        String::from("anotheruser"),
        String::from("anotheruser@another.ji"),
    );
}

struct User {
    is_active: Bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

// 4 Splicing
fn slice_type() {
    // task: function that takes a string of words
    // separated by spaces and returns the first word it finds
    let mut s = String::from("Hello world!");
    let word = first_word(&s);
    s.clear();

    // string slices
    let s = String::from("these are some words");
    let w1 = &s[0..5];
    let w2 = &s[6..9];
    println!("w1: {w1} w2: {w2}");

    let first_word = first_word_slices(&s);
    println!("first_word:{}", first_word);
}

fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // byte literal syntax
            return i; // i is the position
        }
    }
    s.len()
}

/* Ownership
 * each value has an owner
 * there can be only one owner
 * if owner goes out of scope, value dropped
 *
 */
fn ownership() {
    // string literal, fix size, immutable, stack
    let _s1 = "Hello";

    // the string type CAN be mutated, heap
    let mut s = String::from("Hello");

    s.push_str(", world!"); // append
    println!("{}", s);
    ownership_move();

    // return multiple valies: use a value but not taking ownership
    let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("length of '{}' is {}", s2, len);

    let len = calculate_length_with_reference(&s1);
    println!("** with reference: length of '{}' is {}", s1, len);

    change_reference(&mut s);
    println!("{}", s);

    // ownership and functions
    // let s = String::from("Hello");
    // takes_ownership(s);
    // println!("s here not exists anymore: {s}");
}

fn change_reference(s: &mut String) {
    s.push_str(", world changing mutable reference");
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len() // nincs ; !! mertt akkr nem adja vissza
} // s dropped here, but s is a reference

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_ownership(my_string: String) {
    println!("Ownership taken: {}", my_string);
} // string dropped

fn _takes_and_gives_back(str: String) -> String {
    str // ide nem kell; mert akkor nem adja vissza!!
}

fn ownership_move() {
    let x = 5;
    let _y = x; // copy the value of x and bind it to y

    let s1 = String::from("hello");
    let s2 = s1.clone(); // data on the stack (ptr, len, capacity) copied
                         // but the values on the heap not
                         // points to the same heap
                         //
    println!("s1 cloned into s2; print s1: {}, s2: {})", s1, s2);
}

// convert Fahrenheit to celsius
// Generate nth fibonacci numbers

fn for_loop() {
    let numbers = [1, 2, 3, 4, 5];

    for n in numbers {
        println!("element: {n}");
    }

    for x in (1..4).rev() {
        println!("counting with rev and for range: {x}");
    }
}

fn counting() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(" the result is {result}");
}

fn counting_old_way() {
    let mut counter: i32 = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    print!("old way: {counter}");
}

// Control Flow
fn control_flow() {
    let number = 15;

    // if is an expression
    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("osztható 3-mal");
    } else if number % 5 == 0 {
        println!("osztható 5 tel is");
    } else {
        println!("condition as false")
    }
    let res = if number % 10 == 0 { 3 } else { 4 };
    println!("result is {}", res);
}

// functions with return value
// return value is the value of the final expression implicitly
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // ide nemkell ; mert akkor statement lesz és ()-t ad
}

fn another_function(_x: i32) {
    println!("I am an other function");
}

// lesson 1
fn variables() {
    // variables and mutability
    let mut x = 5;
    println!("Value of x is {x}");
    x = 6;
    println!("Value of x is {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("konstans: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    println!("y is {y}");
    let y = 6;
    println!("y is {y}");

    // Data types: scalar  and compound
    // integer types: i8, i16, i32, i64, i128, isize
    // floating point f32, f64
    let x = 2.305;
    let y: f32 = 3.5051;

    let sum = x + y;
    let _diff = x - y;
    let _product = x * y;
    let _quotient = x / y;
    let _truncated = x / 4.0;
    let _remainder = x % 2.0;
    let complex = (2.0 * x + 5.0) / (y - 3.0);
    println!("floatings: {x} and {y} sum: {sum} complex:{complex}");

    // boolean
    let _t = true;
    let _f: bool = false; // explicit type annotation

    // charecter type
    let _c = 'z'; // single  quotes!

    // compound types:tuples and arrays
    let _tup: (i32, f64, u8) = (500, 6.4, 1); // fixed length

    let tupi = (100, 7.2, 'z');
    let (x, _y, z) = tupi; // destructuring. breaks the single tuple
    println!(" tuple values: x: {x} z: {z}");

    // array type: fixed length  same type collection of elements
    let _a = [1, 2, 3, 4, 5, 6]; // allocated on the stack
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5]; // same element 5times
                     //
                     // code sample
    let a = [1, 2, 3, 4, 5];
    println!("Enter index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];
    println!("selected value:{element} at index:{index}");
}
