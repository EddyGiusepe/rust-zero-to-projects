/*
Senior Data Scientist/AI Engineering.: Dr. Eddy Giusepe Chirinos Isidro

Run
===
rustc 3_rust_variables.rs -o rust_variables && ./rust_variables

Format the code
---------------
rustfmt 3_rust_variables.rs


OBS: By default, in rust the value of the variable is immutable, or cannot be changed.
*/
fn main() {
    let name = "Katty";
    println!("The first name of my sister is: {}", name);
    println!("I am Katty's brother and my age is: {}", 45);

    println!("\n");
    // Change variable values with "mut" keyword:
    let mut age = 38;
    println!("My age is: {}", age);
    age = 45;
    println!("After --> my age actually is: {}", age);
}
