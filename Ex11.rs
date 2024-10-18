enum Animal{
    Dog(String),
    Cat(String),
    Fish,
}

fn describe_animal(animal: Animal){
    match animal {
        Animal::Dog(descricao) => println!("Cachorro: {}", descricao),
        Animal::Cat(descricao) => println!("Gato: {}", descricao),
        Animal::Fish => println!("Peixe: não possui uma descrição")
    }
}

fn main(){
    let dog = Animal::Dog("Animal de quatro patas, peludo e bem amoroso".to_owned());
    describe_animal(dog);
}