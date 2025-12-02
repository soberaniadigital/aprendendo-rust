use clap::Parser; //Importando o analisador da biblioteca clap

#[derive(Parser)]
#[command(
    name = "CLI Básica",
    version = "1.0",
    author = "João Filipe",
    about = "Um exemplo de CLI simples em Rust"
)]
struct Args { //Args de Arguments

    #[arg(short, long)] //Permite chamar o argumento usando um nome curto (short), e também seu nome por extenso
    argumento: Option<String>, //Option pois o usuario pode não digitar nada, e então será None

    #[arg(short, long)]
    opcional: bool, //Argumento não obrigatório. Será True se for usado
}

fn main() {
    let args = Args::parse(); //Declarando um Args, e usando parse() para analisar os argumentos

    println!("=====CLI=====\n");

    if args.opcional {
        println!("Você adicionou um argumento opcional, que executará alguma função específica");
    }

    match args.argumento {
        Some(argumento) => println!("O argumento, que geralmente é o nome do comando, foi processado.\n\n Argumento usado: {:?}", argumento),
        None => println!("Tente: cargo run -- -a ARGUMENTO\n OU \n cargo run -- --help"),
    }
}
