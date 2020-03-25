pub fn run(){

    // normal print line to command line
    println!("hello world!");

    // printing ints in rust
    println!("this is a number: {}", 10);

    // two placeholders
    println!("my name is {}, i am {} years old", "Abdulsalam", 20);

    // index placeholders positional arguments
    println!("Hi {2}, my name is {0} {1}. {2} do you like to {3}?", "Abdulsalam", "Aboubakar", "Ali", "code");

    // named arguments
    println!("My name is {name} and I like to play {activity}", name = "abdul", activity = "football");

    // placeholder traits
    println!("Binary value: {:b}. \nHex value: {:x}. \nOctal value: {:o}.", 11,11,11);

    // debugging placeholder
    println!("{:?}",(12,true,"hello"));

    // basic calculations
    println!("2 + 5 = {}", 2+5 );
}