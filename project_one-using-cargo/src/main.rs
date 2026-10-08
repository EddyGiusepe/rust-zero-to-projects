/*
Senior Data Scientist/AI Engineering.: Dr. Eddy Giusepe Chirinos Isidro

Run
===
cargo run

Format the code
---------------
cargo fmt
*/
fn main() {
    let name = "Rustzinho";
    let language = "Rust";
    println!("Hello, {}! Welcome to the world of {}", name, language);
    println!("Version the program: {}", env!("CARGO_PKG_VERSION"));
}
