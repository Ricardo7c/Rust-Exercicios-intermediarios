# Structs, tuple structs, enums e matching.

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

### Exercício 7: Structs Aninhadas

Crie duas structs: `Pessoa` (com os campos `nome` e `endereco`) e `Endereco` (com os campos `rua`, `numero` e `cidade`). Instancie uma `Pessoa` com um `Endereco` e imprima as informações de ambas as structs.

### Exercício 8: Enum e Structs Combinados

Crie um enum `Veiculo` com as variantes `Carro` e `Moto`, onde `Carro` contém uma struct `DetalhesCarro` e `Moto` contém uma struct `DetalhesMoto`. Escreva uma função que recebe um `Veiculo` e usa `match` para imprimir as informações detalhadas de cada variante.

Esses exercícios vão ajudar a reforçar seu aprendizado sobre structs, tuple structs, enums e matching. Boa prática!

### Exercício 9: **Sistema de Notificações**

Crie um `enum` chamado `Notification` que pode representar três tipos diferentes de notificações: `Email`, `SMS` e `PushNotification`. Cada variante deve conter diferentes informações:

- `Email`: endereço de email e assunto da mensagem.
- `SMS`: número de telefone e conteúdo do SMS.
- `PushNotification`: nome do aplicativo e conteúdo da notificação.

Implemente uma função chamada `send_notification` que recebe uma `Notification` e imprime a mensagem apropriada dependendo do tipo de notificação.

### Exercício 10: **Estados de Pagamento**

Crie um `enum` chamado `PaymentStatus` que represente os estados de um pagamento:

- `Pending`: pagamento pendente.
- `Completed`: pagamento completo com o valor pago.
- `Failed`: pagamento falhou com uma mensagem de erro.

Implemente uma função chamada `print_status` que recebe um `PaymentStatus` e imprime o estado atual do pagamento.

### Exercício 11: **Tipos de Animal**

Crie um `enum` chamado `Animal` que pode representar diferentes tipos de animais:

- `Dog`: com uma string representando o nome.
- `Cat`: com uma string representando o nome.
- `Fish`: sem dados adicionais.

Implemente uma função chamada `describe_animal` que recebe um `Animal` e imprime uma descrição do animal com base em seu tipo.

### Exercício 12: **Sistema de Resultados**

Implemente um `enum` chamado `Result` para representar os possíveis resultados de uma operação:

- `Success`: contendo o valor do resultado.
- `Error`: contendo uma mensagem de erro.

Implemente uma função chamada `process_result` que recebe um `Result` e realiza uma ação diferente dependendo de ser sucesso ou erro, imprimindo o valor ou a mensagem de erro.

### Exercício 13: **Controle de Tráfego**

Crie um `enum` chamado `TrafficLight` que represente os três sinais de um semáforo:

- `Red`
- `Yellow`
- `Green`

Implemente uma função chamada `next_light` que recebe um `TrafficLight` e retorna o próximo sinal (por exemplo, se for `Red`, deve retornar `Green`).

### Exercício 14: Função Comparação de Strings
Implemente uma função chamada `maior_str` que recebe duas referências para strings e retorna a maior delas (aquela com o maior comprimento). Use *lifetimes* para garantir que a função retorne uma referência que seja válida enquanto ambas as referências de entrada forem válidas.

#### Requisitos:
- A função deve receber duas referências para `&str` e retornar uma delas.
- O compilador precisa saber que a referência retornada deve ter o mesmo tempo de vida que as entradas.


### Exercício 15: Struct com Referências
Crie uma struct chamada `Pessoa` que contenha referências para um nome e uma cidade, ambos do tipo `&str`. Depois, implemente uma função `informacoes` que receba uma instância de `Pessoa` e retorne uma string formatada com o nome e a cidade dessa pessoa.

#### Requisitos:
- A struct `Pessoa` deve armazenar referências, e os *lifetimes* dessas referências devem ser especificados corretamente.
- A função `informacoes` deve aceitar uma referência para a `Pessoa` e retornar uma nova string com as informações.
