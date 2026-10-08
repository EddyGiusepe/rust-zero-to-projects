/*
Senior Data Scientist/AI Engineering.: Dr. Eddy Giusepe Chirinos Isidro

Run
===
cargo run

Format the code
---------------
cargo fmt
*/
use colored::*;

fn somar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultado: i32 = somar(3, 4);
    println!("{}", "A seguir vamos a imprimir o resultado:".green());
    println!("3 + 4 = {}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_somar() {
        assert_eq!(somar(2, 3), 5);
        assert_eq!(somar(-1, 1), 0);
        assert_eq!(somar(0, 0), 0);
        assert!(somar(2, 3) >= 5);
    }

    #[test]
    fn teste_somar_negativos() {
        assert_eq!(somar(-5, -3), -8);
    }
}
