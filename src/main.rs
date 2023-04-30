use std::io;

// terrible colors, 
// update locally
fn main() {
    another_function(5);
    variables();

}

fn another_function (_x: i32 ){
    println!("I am an other function");
}

// lesson 1
fn variables(){
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
    let _diff = x-y;
    let _product = x * y;
    let _quotient = x / y;
    let _truncated  = x/4.0;
    let _remainder = x % 2.0;
    let complex = (2.0 * x+ 5.0)/(y-3.0);
    println!("floatings: {x} and {y} sum: {sum} complex:{complex}");

    // boolean
    let _t = true;
    let _f: bool = false; // explicit type annotation
    
    // charecter type
    let _c = 'z'; // single  quotes!

    // compound types:tuples and arrays
    let _tup: (i32,f64, u8)= (500, 6.4, 1); // fixed length
    
    let tupi = (100, 7.2, 'z');
    let (x,_y,z) = tupi; // destructuring. breaks the single tuple
    println!(" tuple values: x: {x} z: {z}");

    // array type: fixed length  same type collection of elements
    let _a = [1,2,3,4,5,6]; // allocated on the stack
    let _b: [i32; 5] = [1,2,3,4,5];
    let _c = [3;5]; // same element 5times
                    //
    // code sample
    let a = [1,2,3,4,5];
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
