//=============================================
// Exemplo 1
//=============================================

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Tenta abrir o arquivo "hello.txt"
    // O retorno é Result<File, Error>
    let f = File::open("hello.txt");

    // Faz o tratamento de erros usando match
    let f = match f {
        // Caso dê certo, simplesmente retorna o arquivo
        Ok(file) => file,

        // Caso o erro seja "arquivo não encontrado", tentamos criar o arquivo
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                // Sucesso na criação → retornamos o novo arquivo
                Ok(fc) => fc,
                // Falha na criação → panic com mensagem detalhada
                Err(e) => {
                    panic!(
                        "Tentou criar um arquivo e houve um problema: {:?}",
                        e
                    );
                }
            }
        }

        // Qualquer outro tipo de erro ao abrir o arquivo
        Err(error) => {
            panic!(
                "Houve um problema ao abrir o arquivo: {:?}",
                error
            );
        }
    };

    // Aqui já temos um File funcional chamado `f`
    // Você poderia usar `f` para leitura/escrita
}
