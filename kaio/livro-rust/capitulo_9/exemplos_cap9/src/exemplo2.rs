//=============================================
// Exemplo 2
//=============================================

/*
use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    // Tenta abrir o arquivo "mensagem.txt"
    let mut arquivo = match File::open("mensagem.txt") {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            // Se o arquivo não existe, cria um novo
            match File::create("mensagem.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Erro ao criar o arquivo: {:?}", e);
                }
            }
        },
        Err(e) => {
            // Qualquer outro erro ao abrir
            panic!("Erro ao abrir o arquivo: {:?}", e);
        }
    };

    // Cria uma String para armazenar o conteúdo
    let mut conteudo = String::new();

    // Tenta ler o conteúdo do arquivo
    match arquivo.read_to_string(&mut conteudo) {
        Ok(_) => println!("Conteúdo do arquivo:\n{}", conteudo),
        Err(e) => println!("Erro ao ler o arquivo: {:?}", e),
    }
}
*/