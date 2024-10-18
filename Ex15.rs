struct Pessoa<'a>{
    nome: &'a str,
    cidade: &'a str
}

fn informacoes<'a>(pessoa: &'a Pessoa) -> String{
    format!("Nome: {}, Cidade: {}", pessoa.nome, pessoa.cidade)
}

fn main(){
    let ricardo = Pessoa{
        nome: "Ricardo",
        cidade: "São Paulo"
    };

    let info = informacoes(&ricardo);
    println!("{}", info);
}