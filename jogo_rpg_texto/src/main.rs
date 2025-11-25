use std::io::{self};

use crate::{game::game, personagens::Personagem};

mod personagens;
mod itens;
mod combate;
mod game;
mod salas;

fn main() 
{
    let mut barbaro = Personagem::gerar_barbaro();
    let mut arqueiro = Personagem::gerar_arqueiro();
    let mut mago = Personagem::gerar_mago();

    println!("Escolha o seu personagem\n");
    println!("1 --> Nome: {} == Vida: {} == Ataque: {} == Defesa: {}", barbaro.nome, barbaro.vida, barbaro.ataque, barbaro.defesa);
    println!("2 --> Nome: {} == Vida: {} == Ataque: {} == Defesa: {}", mago.nome, mago.vida, mago.ataque, mago.defesa);
    println!("3 --> Nome: {} == Vida: {} == Ataque: {} == Defesa: {}", arqueiro.nome, arqueiro.vida, arqueiro.ataque, arqueiro.defesa);

    let mut escolha = String::new();

    io::stdin().read_line(&mut escolha).expect("Deu merda");
    
    match escolha.as_str().trim()
    {
        "1" => {game(&mut barbaro);},
        "2" => {game(&mut mago);},
        "3" => {game(&mut arqueiro);},
        _   => {println!("Some daqui irmão!");},
    }
}
