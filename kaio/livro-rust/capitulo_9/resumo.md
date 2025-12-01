# Resumo do Capítulo 9 do Rust Book

O Capítulo 9 do Rust Book trata do **tratamento de erros em Rust**, mostrando como lidar de forma segura e explícita com situações em que algo pode dar errado durante a execução do programa. Diferente de outras linguagens que usam exceções, Rust opta por soluções que forçam o programador a decidir como tratar os erros, evitando crashes inesperados.

## Mecanismos de tratamento de erros

O capítulo apresenta dois mecanismos principais para lidar com falhas:

### 1. `panic!`
Usado para **erros irrecuperáveis**, quando o programa não consegue continuar de forma segura. Um exemplo típico é acessar um índice fora de um vetor. Quando `panic!` é chamado, o programa interrompe a execução imediatamente, exibindo uma mensagem de erro. Por isso, seu uso é recomendado apenas para situações críticas.

### 2. `Result<T, E>`
Usado para **erros recuperáveis**. `Result` é um enum com duas variantes:  
- `Ok(T)` para indicar sucesso  
- `Err(E)` para indicar erro  

Ao usar `Result`, o programador precisa tratar explicitamente os casos de sucesso e de erro, normalmente usando `match`, métodos como `unwrap` ou `expect`, ou o operador `?` para propagar erros. Esse padrão permite que o programa reaja a falhas, por exemplo, tentando criar um arquivo que não existe ou avisando o usuário sobre problemas de entrada/saída.

## Exemplos práticos

O capítulo também explora exemplos práticos de:  
- Abertura e criação de arquivos  
- Leitura de conteúdo  
- Tratamento de erros  

Ele demonstra como escrever código robusto, lidando com situações como arquivos inexistentes, erros de leitura ou falhas na criação de arquivos, sempre garantindo que o programa não quebre de forma inesperada.

## Conclusão

O Capítulo 9 ensina que o Rust força uma abordagem consciente ao lidar com erros: usar `panic!` apenas para situações críticas e `Result` para falhas que o programa pode recuperar. Esse modelo garante programas mais **seguros e previsíveis**, tornando a detecção e o tratamento de problemas parte integrante do fluxo de desenvolvimento.
