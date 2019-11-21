#![allow(dead_code)]
use std::mem;

pub fn main() {
    // Numbers
    // i8 u8 i16 u16 i32 u32
    let mut a:u16 = 27700;
    println!("Value is: {} and size is {} bytes", a, mem::size_of_val(&a));
    a = 225;
    println!("Value is: {} and size is {} bytes", a, mem::size_of_val(&a));

    // usize isize assign the memory addrsss that is your memory address on the system
    let mut b:usize = 27700;
    println!("Value is: {} and size is {} bytes and {}-bit-OS", b, mem::size_of_val(&b), mem::size_of_val(&b)*8);

    // Text
    let name = "Taqi";
    println!("Value is: {} and size is {} bytes", name, mem::size_of_val(&name));

    // Boolean
    let married = false;
    println!("Value is: {} and size is {} bytes", married, mem::size_of_val(&married));

    // Arithmetic
    // let age_cubed = i32::powi(2, 3);
    // println!("{} cubed is {}", 2, age_cubed);
    // let height_cubed = f32::powf(6.3, 3.2);
    // println!("{} cubed is {}", 6.3, age_cubed);

    // Bitwise
    let three = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1|2 = {}", three);  // 01 OR 10 = 11 == 3 10
}
