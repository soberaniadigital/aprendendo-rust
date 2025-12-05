//Importando o analisador da biblioteca clap
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "CLI Básica",
    version = "1.0",
    author = "João Filipe",
    about = "Um exemplo de CLI simples em Rust"
)]
//Args de Arguments
struct Args { 

    //Permite chamar o argumento usando um nome curto (short), e também seu nome por extenso
    #[arg(short, long)]
    //Option pois o usuario pode não digitar nada, e então será None
    argumento: Option<String>,

    #[arg(short, long)]
     //Argumento não obrigatório. Será True se for usado
    opcional: bool,
}

fn main() {
    //Declarando um Args, e usando parse() para analisar os argumentos
    let args = Args::parse(); 

    println!("=====CLI=====\n");

    if args.opcional {
        println!("Você adicionou um argumento opcional, que executará alguma função específica");
    }

    match args.argumento {
        Some(argumento) => println!("O argumento, que geralmente é o nome do comando, foi processado.\n\n Argumento usado: {:?}", argumento),
        None => println!("Tente: cargo run -- -a ARGUMENTO\n OU \n cargo run -- --help"),
    }
}
