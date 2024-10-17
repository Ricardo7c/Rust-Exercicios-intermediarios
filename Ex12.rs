enum Sinal{
    Red(String),
    Yellow(String),
    Green(String)
}

fn next_light(x: Sinal) -> Sinal{
    match x {
        Sinal::Red => return Sinal::Yellow,
        Sinal::Yellow => return Sinal::Green,
        Sinal::Green => return Sinal::Red
    }
}

fn main(){
    let red = Sinal::Red("Vermelho!".to_owned());
    let yellow = Sinal::Yellow("Amarelo".to_owned());
    let green = Sinal::Green("Verde".to_owned());

    let proxima = next_light(red);
    println!("{}",proxima);



}