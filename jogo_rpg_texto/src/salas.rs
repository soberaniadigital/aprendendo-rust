use std::process;

use rand::random_range;

use crate::personagens::Personagem;
use crate::combate::combate;
use crate::itens::Item;

pub struct Salas
{
    tem_inimigo: bool,
}

impl Salas
{
    pub fn sala1(&mut self, _player: &mut Personagem)
    {
        println!("
            ╔════════════════════════════════════════════════════════════════╗
            ║                        VILAREJO INICIAL                        ║
            ╚════════════════════════════════════════════════════════════════╝
            
            Você sempre sonhou em se tornar um herói, mas diferente dos outros
            que abandonaram esse sonho, você não o largou, e finalmente na
            idade adulta, está na hora de começar a sua jornada.
            
            Alguns dias atrás, o rei emitiu um alerta para a Guilda de
            aventureiros, a respeito de uma missão de emergência. Um enorme
            dragão vermelho atacou a capital, e sequestrou a princesa,
            levando-a para o leste.
            
            Esta é a sua chance de se tornar o herói que você sempre sonhou.
            ══════════════════════════════════════════════════════════════════");

            println!("Você pode ir para o Leste: (L)");

        // sem inimigo nessa sala
    }
    pub fn sala2(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                        CAMPO ABERTO                ║
            ╚════════════════════════════════════════════════════╝

            Você avança para além do vilarejo e encontra um campo aberto.
            O vento sopra forte, trazendo o cheiro da floresta próxima.
            ══════════════════════════════════════════════════════");

        if self.tem_inimigo 
        {
            let mut goblin = Personagem::gerar_goblin();
            combate(player, &mut goblin);
            self.tem_inimigo = false;

            let mut loopando = 0;
            loop
            {
                let escolha = random_range(1..=3);
                if escolha == 1
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else if escolha == 2
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else
                {
                    let pocao = Item::gerar_poc_vida();
                    player.add_no_inventario(pocao);
                }
                
                if loopando == 1
                {
                    break;
                }
                loopando += 1;
            }
        }

        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }   

    pub fn sala3(&mut self, _player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                       PONTE ANTIGA                 ║
            ╚════════════════════════════════════════════════════╝

            Uma ponte velha e torta cruza um pequeno rio. 
            Você sente que alguém já observou esse lugar.
            ══════════════════════════════════════════════════════");

        // sem inimigo nessa sala
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala4(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                   ENTRADA DA FLORESTA              ║
            ╚════════════════════════════════════════════════════╝

            A floresta começa a se fechar, escurecendo o caminho.
            Rastros pequenos se movem entre os arbustos.
            ══════════════════════════════════════════════════════");

        if self.tem_inimigo 
        {
            let mut goblin = Personagem::gerar_goblin();
            combate(player, &mut goblin);
            self.tem_inimigo = false;

            let mut loopando = 0;
            loop
            {
                let escolha = random_range(1..=3);
                if escolha == 1
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else if escolha == 2
                {
                    let pocao = Item::gerar_poc_defesa();
                    player.add_no_inventario(pocao);
                }
                else
                {
                    let pocao = Item::gerar_poc_vida();
                    player.add_no_inventario(pocao);
                }
                
                if loopando == 1
                {
                    break;
                }
                loopando += 1;
            }
        }
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala5(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                   TRILHA SOMBRIA                   ║
            ╚════════════════════════════════════════════════════╝

            A trilha se estreita, coberta por raízes e galhos.
            O silêncio absoluto domina o lugar.
            ══════════════════════════════════════════════════════");

        if self.tem_inimigo 
        {
            let mut orc = Personagem::gerar_orc();
            combate(player, &mut orc);
            self.tem_inimigo = false;

            let mut loopando = 0;
            loop
            {
                let escolha = random_range(1..=3);
                if escolha == 1
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else if escolha == 2
                {
                    let pocao = Item::gerar_poc_defesa();
                    player.add_no_inventario(pocao);
                }
                else
                {
                    let pocao = Item::gerar_poc_vida();
                    player.add_no_inventario(pocao);
                }
                
                if loopando == 1
                {
                    break;
                }
                loopando += 1;
            }
        }
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala6(&mut self, _player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                  CLAREIRA PERDIDA                  ║
            ╚════════════════════════════════════════════════════╝

            Uma clareira iluminada revela antigas marcas de batalha.
            Parece que aventureiros já passaram por aqui.
            ══════════════════════════════════════════════════════");

        // sem inimigo nessa sala
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala7(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                   CABANA ABANDONADA                ║
            ╚════════════════════════════════════════════════════╝

            Uma cabana velha, caída pela metade, repousa no meio da floresta.
            Algo se mexe lá dentro...
            ═════════════════════════════════════════════════════");

        if self.tem_inimigo 
        {
            let mut orc = Personagem::gerar_orc();
            combate(player, &mut orc);
            self.tem_inimigo = false;

            let mut loopando = 0;
            loop
            {
                let escolha = random_range(1..=3);
                if escolha == 1
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else if escolha == 2
                {
                    let pocao = Item::gerar_poc_defesa();
                    player.add_no_inventario(pocao);
                }
                else
                {
                    let pocao = Item::gerar_poc_vida();
                    player.add_no_inventario(pocao);
                }
                
                if loopando == 1
                {
                    break;
                }
                loopando += 1;
            }
        }
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala8(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                   DESFILADEIRO ESTREITO            ║
            ╚════════════════════════════════════════════════════╝

            As paredes de pedra se fecham sobre você.
            Um único caminho segue em frente.
            ══════════════════════════════════════════════════════");

        if self.tem_inimigo 
        {
            let mut goblin = Personagem::gerar_goblin();
            combate(player, &mut goblin);
            self.tem_inimigo = false;

            let mut loopando = 0;
            loop
            {
                let escolha = random_range(1..=3);
                if escolha == 1
                {
                    let pocao = Item::gerar_poc_ataque();
                    player.add_no_inventario(pocao);
                }
                else if escolha == 2
                {
                    let pocao = Item::gerar_poc_defesa();
                    player.add_no_inventario(pocao);
                }
                else
                {
                    let pocao = Item::gerar_poc_vida();
                    player.add_no_inventario(pocao);
                }
                
                if loopando == 1
                {
                    break;
                }
                loopando += 1;
            }
        }
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala9(&mut self, _player: &mut Personagem) 
    {
        println!("
            ╔════════════════════════════════════════════════════╗
            ║                   PORTÃO DO VALE FINAL             ║
            ╚════════════════════════════════════════════════════╝

            Um portão enorme de pedra bloqueia a saída da floresta.
            Inscrições antigas alertam sobre o perigo à frente.
            ══════════════════════════════════════════════════════");

        // Sem inimigo aqui para criar tensão para a última sala
        println!("Você pode ir para o Leste e Oeste: (L) / (O)");
    }

    pub fn sala10(&mut self, player: &mut Personagem) 
    {
        println!("
            ╔══════════════════════════════════════════════════════════╗
            ║                        COVA DO DRAGÃO                    ║
            ╚══════════════════════════════════════════════════════════╝

            O chão treme a cada passo que você dá. 
            A caverna é iluminada apenas pelo brilho intenso da lava.
            A princesa está acorrentada mais ao fundo.
            O ar fica mais quente e pesado quando você entra.

            A sombra gigantesca de um dragão vermelho surge entre as chamas.
            Este é o momento decisivo. Sua jornada inteira o trouxe até aqui.
            ════════════════════════════════════════════════════════════════");

        // último inimigo: dragão
        if self.tem_inimigo 
        {
            let mut dragao = Personagem::gerar_dragao();
            combate(player, &mut dragao);
            self.tem_inimigo = false;
        }

        println!("
            ╔══════════════════════════════════════════════════════════╗
            ║                        A VITÓRIA FINAL                   ║
            ╚══════════════════════════════════════════════════════════╝

            Com um golpe final, o dragão cai, fazendo a caverna estremecer.
            A princesa agradece emocionada. Seu nome ecoará pelos séculos.
            Você não é mais apenas um sonhador — agora é um herói de verdade.
            ═════════════════════════════════════════════════════════════════");

        process::exit(0);    
    }

    pub fn gerar_sala1() -> Self 
    {
    Self { tem_inimigo: false }
    }
    pub fn gerar_sala2() -> Self 
    {
        Self { tem_inimigo: true } // goblin
    }
    pub fn gerar_sala3() -> Self 
    {
        Self { tem_inimigo: false }
    }
    pub fn gerar_sala4() -> Self 
    {
        Self { tem_inimigo: true } // goblin
    }
    pub fn gerar_sala5() -> Self 
    {
        Self { tem_inimigo: true } // orc
    }
    pub fn gerar_sala6() -> Self 
    {
        Self { tem_inimigo: false }
    }
    pub fn gerar_sala7() -> Self 
    {
        Self { tem_inimigo: true } // orc
    }
    pub fn gerar_sala8() -> Self 
    {
        Self { tem_inimigo: true } // goblin
    }
    pub fn gerar_sala9() -> Self 
    {
        Self { tem_inimigo: false }
    }
    pub fn gerar_sala10() -> Self 
    {
        Self { tem_inimigo: true } // dragão
    } 
}
