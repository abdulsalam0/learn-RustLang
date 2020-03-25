// Variables hold primitive data or references to data
// Variables are imuutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "abdul";

    // change variable to mutable value
    let mut age = 20;
    println!("my name is {}. I am {} years old", name, age);

    age = 21;
    println!("my name is {}. I am {} years old", name, age);

    // define constant 
    const ID: &str = "ADWs21dsawd";

    println!("ID: {}", ID);

    // assigning multiple variables
    let (user_name, user_age) = ("abdul",20);

    println!("{0} is {1}", user_name,user_age);

} 