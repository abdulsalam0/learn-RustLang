/*
Primitive Types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
characters (char)
tuples
Arrays
*/

pub fn run() {

    // set type for variables
    let x: i32 = 1;

    let y: f64 = 1.12312;

    // boolean
    let answer: bool = true;

    // get bool from expression
    let is_greater = 10 > 9;

    // characters
    let letter: char = 'a';
    let emoji: char = '😎';

    println!("x:{}. y:{}. answer:{}.",x,y,answer);

    // printing all values as a tuple 
    println!("{:?}", ( x, y, answer, is_greater, letter, emoji));

    println!("{}", std::i32::MAX);
    println!("{}", std::i64::MAX);


}