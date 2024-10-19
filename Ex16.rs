mod ex16_mod;

use ex16_mod::*;

fn main(){
    let a = 6;
    let b = 2;
    println!("Os numeros são: {} e {}", a, b);
    println!("Adição: {}", soma(a, b));
    println!("Subtração: {}", sub(a, b));
    println!("Divisão: {}", div(a, b));
    println!("Multiplicação: {}", mult(a, b));
}