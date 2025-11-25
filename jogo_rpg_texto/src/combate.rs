use std::io;
use std::process;

use crate::{personagens::Personagem};

pub fn combate (player: &mut Personagem, inimigo: &mut Personagem)
{
    let mut decisao = true;

    println!("\n========================");
    println!("Um {} apareceu! == Vida: {} ==", inimigo.nome, inimigo.vida);
    println!("Você tem: {} de vida.", player.vida);
    println!("========================\n");

    while player.vida > 0 && inimigo.vida > 0 
    {

        println!("========================");
        println!("O que você irá fazer?");
        println!("========================\n");

        println!("1. Atacar");
        println!("2. Golpe Especial {}/5", player.ataque_especial.unwrap_or(0));
        println!("3. Usar Item\n");

        let mut escolha = String::new();

        io::stdin().read_line(&mut escolha).expect("Não foi filhote");
        
        let escolha = escolha.as_str().trim();

        match escolha {

            "1" => { player.ataque(inimigo, false);
                    println!("\n========================");
                    println!("Você atacou!");
                    println!("O {} também te atacou!", inimigo.nome);
                    println!("========================\n");
                },

            "2" => { player.ataque(inimigo, true);
                    println!("\n========================");
                    println!("Você usou o golpe especial!");
                    println!("O {} também te atacou!", inimigo.nome);
                    println!("========================\n");
                 },

            "3" => { 
                        println!("\n========================");
                        println!("Qual item deseja utilizar?\n");
                        let tem = player.mostrar_inventario();
                        println!("========================\n");

                        if tem
                        {
                            let mut item_escolhido = String::new();

                            io::stdin().read_line(&mut item_escolhido).unwrap();
                            println!("");

                            if item_escolhido.trim().parse::<i32>().is_ok()
                            {
                                let item_escolhido: i32 = item_escolhido.trim().parse().unwrap();

                                println!("========================");
                                player.usar_item( item_escolhido-1);
                                println!("========================\n");
                            }
                            else
                            {
                                println!("\n========================");
                                println!("Errou filho");
                                println!("========================\n");
                                
                            }
                        }
                        else
                        {
                            println!("\n========================");
                            println!("Tem nada aqui não, jão!");
                            println!("========================\n");    
                        }    
                 
                },
            
            _ =>    {
                    println!("\n========================"); println!("Errou o número, perdeu a vez."); 
                    println!("========================\n"); println!("Você foi atacado!"); },

        }

        inimigo.ataque(player, false);

        if inimigo.vida <= 0
        {
            inimigo.vida = 0; 
        }
        if player.vida <= 0
        {
            player.vida = 0;
        }

        println!("Sua vida: {}\n", player.vida);
        println!("Vida do Inimigo {}\n", inimigo.vida);

        if inimigo.vida <= 0 && player.vida <= 0
        {
            inimigo.vida = 0;
            println!("\n========================");
            println!("Você perdeu!");
            println!("========================\n");
            decisao = false;
            continue;
        }
        else if inimigo.vida <= 0
        {
            inimigo.vida = 0;
            println!("\n========================");
            println!("Você venceu!");
            println!("========================\n");
            decisao = true;
            continue;
        }
        else if player.vida <= 0
        {
            inimigo.vida = 0;
            println!("\n========================");
            println!("Você perdeu!");
            println!("========================\n");
            decisao = false;
            continue;
        }

    }

    if decisao 
    {
        println!("========================");
        println!("Combate encerrado\n");
        println!("Vida restante: {}", player.vida);
        println!("========================\n");
    }
    else
    {
        println!("========================");
        println!("Combate encerrado\n");
        println!("Tente novamente!");
        println!("========================\n");
        process::exit(0);
    }
    
}