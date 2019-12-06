#![allow(dead_code)]
#![allow(unused_variables)]
// mod types_and_variables;
// mod control_flow;


fn main() {
    // types_and_variables::main();
    // control_flow::main();
    check_positive_negative_zero(55);
    print_arguments(25, 5.4, false);
    println!("{:#?}", square_the_number(4));
}


// Q # 1. Write a rust program, define a function that receives one argument of any suitable data
//        type and print whether the number is positive, negative or equal to zero. (hint: if/else)
fn check_positive_negative_zero(num: i32){
    if num > 0 {
        println!("number {} is positive", num);
    } else if num < 0 { 
        println!("number {} is positive", num);
    } else {
        println!("number {} is zero", num);
    }
}

// Q # 2. Write a rust program, define a function that receives 3 arguments of different data types
//        (integer, float, boolean) and print them on the console.
fn print_arguments(age: i32, height: f32, married: bool ){
    let maritial_status = if married {"married"} else {"single"};
    println!("Your Age is {}, height is {} and your are {}", age, height, maritial_status);
}

// Q # 3. Write a rust program, define a function that receives a number and return the number
//        itself and its square by using tuple.
fn square_the_number(number: i32)->(i32, i32){
    (number, number*number)
}


