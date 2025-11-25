use std::io;

use crate::{personagens::Personagem, salas::Salas};

pub fn game(player: &mut Personagem)
{
    let mut sala_atual = 1;
    let mut andar = String::new();

    let mut sala1 = Salas::gerar_sala1();
    let mut sala2 = Salas::gerar_sala2();
    let mut sala3 = Salas::gerar_sala3();
    let mut sala4 = Salas::gerar_sala4();
    let mut sala5 = Salas::gerar_sala5();
    let mut sala6 = Salas::gerar_sala6();
    let mut sala7 = Salas::gerar_sala7();
    let mut sala8 = Salas::gerar_sala8();
    let mut sala9 = Salas::gerar_sala9();
    let mut sala10 = Salas::gerar_sala10();

    loop
    {
        match sala_atual 
        {
            1   => sala1.sala1(player),
            2   => sala2.sala2(player),    
            3   => sala3.sala3(player),
            4   => sala4.sala4(player),    
            5   => sala5.sala5(player),
            6   => sala6.sala6(player),    
            7   => sala7.sala7(player),
            8   => sala8.sala8(player),    
            9   => sala9.sala9(player),
            10  => sala10.sala10(player), 
            _   => {},   
        }
        
        andar.clear();
        io::stdin().read_line(&mut andar).expect("Não deu filhote");

        // ─────────── Sala 1 ───────────
        if sala_atual == 1 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        // ─────────── Sala 2 ───────────
        else if sala_atual == 2 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 2 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 3 ───────────
        else if sala_atual == 3 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 3 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 4 ───────────
        else if sala_atual == 4 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 4 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 5 ───────────
        else if sala_atual == 5 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 5 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 6 ───────────
        else if sala_atual == 6 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 6 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 7 ───────────
        else if sala_atual == 7 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 7 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 8 ───────────
        else if sala_atual == 8 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 8 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 9 ───────────
        else if sala_atual == 9 && andar.as_str().trim().to_uppercase() == "L"
        {
            sala_atual += 1;
            continue;
        }
        else if sala_atual == 9 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Sala 10 ───────────
        else if sala_atual == 10 && andar.as_str().trim().to_uppercase() == "O"
        {
            sala_atual -= 1;
            continue;
        }
        // ─────────── Direção inválida ───────────
        else
        {
            println!("Você não pode ir nessa direção.");
        }
    }
}

