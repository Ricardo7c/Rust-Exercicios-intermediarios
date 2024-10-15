# Rust-Exercicios-intermediarios
Exercicios gerados pelo chat GPT com foco em Structs, Tuple structs, Enums e Matching.

### Exercício 1: Definição e Instanciação de Structs

Crie uma struct chamada `Livro` com os seguintes campos: `titulo`, `autor` e `paginas`. Em seguida, instancie um livro e imprima suas informações.

### Exercício 2: Struct com Métodos

Adicione um método `resumo` à struct `Livro`, que retorna uma string contendo o título e o autor do livro. Instancie um livro e chame esse método para exibir o resumo.

### Exercício 3: Tuple Structs

Crie uma tuple struct chamada `Coordenada` que contenha dois valores `f64`, representando a latitude e a longitude. Em seguida, crie uma função que recebe uma `Coordenada` e imprime os valores.

### Exercício 4: Atualização de Campos de Struct

Dada a struct `Carro` com os campos `modelo`, `ano` e `preco`, escreva uma função que recebe um carro e altera o valor do campo `preco`.

### Exercício 5: Enum Simples

Crie um enum chamado `EstadoLampada` com as variantes `Ligada` e `Desligada`. Escreva uma função que recebe um `EstadoLampada` e imprime o estado atual da lâmpada.

### Exercício 6: Enum com Dados

Defina um enum `Mensagem` com as variantes `Texto(String)`, `Imagem(String, u32, u32)`, e `Video(String, u32)`. Crie uma função que recebe uma `Mensagem` e imprime informações baseadas na variante recebida.

### Exercício 7: Matching com Enum

Use o enum `Mensagem` do exercício anterior e escreva uma função que usa `match` para lidar com cada variante de forma diferente, exibindo os dados correspondentes.

### Exercício 8: Structs Aninhadas

Crie duas structs: `Pessoa` (com os campos `nome` e `endereco`) e `Endereco` (com os campos `rua`, `numero` e `cidade`). Instancie uma `Pessoa` com um `Endereco` e imprima as informações de ambas as structs.

### Exercício 9: Matching com Tuples

Dada uma tuple `(i32, &str)`, escreva uma função que usa `match` para verificar o valor do primeiro item e imprime mensagens diferentes com base no valor.

### Exercício 10: Enum e Structs Combinados

Crie um enum `Veiculo` com as variantes `Carro` e `Moto`, onde `Carro` contém uma struct `DetalhesCarro` e `Moto` contém uma struct `DetalhesMoto`. Escreva uma função que recebe um `Veiculo` e usa `match` para imprimir as informações detalhadas de cada variante.

Esses exercícios vão ajudar a reforçar seu aprendizado sobre structs, tuple structs, enums e matching. Boa prática!
