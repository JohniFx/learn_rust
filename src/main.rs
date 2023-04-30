use std::io;
/*
 *  Statements and expressions
 *  Rust is an expression based language
 *  statements dont evaluate to a value, and expressed as ()
 * */

fn main() {
    for_loop();
    counting();
    counting_old_way();
    another_function(5);
    variables();

    // expression does not have semicolon at the end
    // expressions evaluate to a value
    // statement has a semicolon at the end  and no return value
    let y = {
        let x = 3;
        x + 1
    };
    let f = five();
    println!("five return: {f}");
    let p1 = plus_one(f);
    println!("five return: {p1}");
    control_flow();
}

// convert Fahrenheit to celsius
// Generate nth fibonacci numbers


fn for_loop(){
    let numbers = [1,2,3,4,5];

    for n in numbers{
        println!("element: {n}");
    }

    for x in (1..4).rev(){
        println!("counting with rev and for range: {x}");
    }
}

fn counting(){
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 10{
            break counter *2;
        }
    };
    println!(" the result is {result}");
}

fn counting_old_way(){
    let mut counter: i32 = 0;

    loop{
        counter += 1;
        if counter == 10{
            break;
        }
    }
    print!("old way: {counter}");
}

// Control Flow
fn control_flow(){
    let number = 15;

    // if is an expression
    if number < 5{
        println!( "condition was true");
    } else if number % 3 == 0{
        println!("osztható 3-mal");
    } 
    else if number % 5 == 0{
        println!("osztható 5 tel is");
    }
    else{

        println!( "condition as false")
    }
    let res = if number % 10 == 0 {3} else {4};
    println!("result is {}", res);
}

// functions with return value
// return value is the value of the final expression implicitly
fn five()-> i32{
    5
}

fn plus_one(x: i32)-> i32{
    x + 1 // ide nemkell ; mert akkor statement lesz és ()-t ad
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
