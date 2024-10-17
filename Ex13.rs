fn maior_str<'a>(a: &'a str, b:&'a str) -> &'a str{
    if a.len() > b.len(){
        a
    }else{
        b
    }
}

fn main(){
    let a = "Rust";
    let b = "Language";
    let resultado = maior_str(&a, &b);
    println!("{}",resultado);
}