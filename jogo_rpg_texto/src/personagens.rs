use crate::itens::Item;

pub struct Personagem 
{
    pub nome: String,
    pub vida: i32,
    pub ataque: u32,
    pub ataque_especial: Option<i32>,
    pub defesa: u32,
    pub inventario: Vec<Item>,
}

impl Personagem 
{
    //Ataque do personagem
    pub fn ataque (&mut self, alvo: &mut Personagem, especial: bool) 
    {

        let mut margem: u32 = 0;

        match self.nome.as_str()
        {

            "Arqueiro"  => { margem = 31; },
            "Bárbaro"   => { margem = 26; },
            "Mago"      => { margem = 36; },
            "Dragão"    => { margem = 50; },
            "Goblin"    => { margem = 15; },
            "Orc"       => { margem = 20; },
            _           => {println!("aoba");},
        }

        if self.ataque_especial > Some(0) && especial == true {

            if let Some(v) = &mut self.ataque_especial 
                {
                *v -= 1; //Gastar ataque especial
                };

            margem -=15;

        } else {

            if especial == true {
                println!("O ataque especial falhou, e se tornou um golpe comum!\n");
            }
        }

        let mut dano = rand::random_range(1..=margem);
        
        if self.ataque_especial > Some(0) && especial == true {
           
                dano = dano*2 + self.ataque;
        }

        if dano > alvo.defesa
        {
            alvo.vida -= dano as i32;
        }
    }

    //inventario
    pub fn mostrar_inventario(&self) -> bool
    {
        let mut indice = 0;
        if !self.inventario.is_empty()
        {
            for itens in &self.inventario
            {
                println!("{}. Nome: {} , Descrição: {}", (indice+1), itens.nome, itens.descrição);
                indice += 1;
            }
            return true;
        }
        else
        {
            println!("Inventário está vazio");
            return false;
        }
    }

    //inventario q consome o item (ownership), no caso o item some e o invetario toma posse
    pub fn add_no_inventario(&mut self, item: Item)
    {
        println!("Item adicionado. Nome: {}", item.nome);
        self.inventario.push(item);
    }

    //usando
    pub fn usar_item(&mut self, indice: i32)
    {
        if indice >= 0 && indice < self.inventario.len() as i32
        {
            let item = self.inventario.remove(indice as usize);
            if item.nome == "Poção_de_ataque"
            {
                self.ataque += item.efeito;
                println!("Poção de ataque usada!");
            }
            else if item.nome == "Poção_de_vida"
            {
                self.vida += item.efeito as i32;
                println!("Poção de vida usada!");
            }
            else if item.nome == "Poção_de_defesa"
            {
                self.defesa += item.efeito;
                println!("Poção de defesa usada!");
            }
        }
        else
        {
            println!("Índice inválido.")
        }
    }

    //cria barbaro
    pub fn gerar_barbaro () -> Self
    {
        Self 
        {
            nome: "Bárbaro".to_string(),
            vida: 300,
            ataque: 15,
            ataque_especial: Some(5),
            defesa: 12,
            inventario: Vec::new(),
        }
    }

    //cria arqueiro
    pub fn gerar_arqueiro () -> Self
    {
        Self 
        {
            nome: "Arqueiro".to_string(),
            vida: 250,
            ataque: 12,
            ataque_especial: Some(5),
            defesa: 7,
            inventario: Vec::new(),
        }
    }

    //cria mago
    pub fn gerar_mago () -> Self
    {
        Self 
        {
            nome: "Mago".to_string(),
            vida: 260,
            ataque: 20,
            ataque_especial: Some(5),
            defesa: 5,
            inventario: Vec::new(),
        }
    }

    //cria dragao
    pub fn gerar_dragao () -> Self
    {
        Self 
        {
            nome: "Dragão".to_string(),
            vida: 125,
            ataque: 10,
            ataque_especial: None,
            defesa: 10,
            inventario: Vec::new(),
        }
    }

    //cria goblin
    pub fn gerar_goblin () -> Self
    {
        Self 
        {
            nome: "Goblin".to_string(),
            vida: 40,
            ataque: 10,
            ataque_especial: None,
            defesa: 2,
            inventario: Vec::new(),
        }
    }

    //cria orc
    pub fn gerar_orc () -> Self
    {
        Self 
        {
            nome: "Orc".to_string(),
            vida: 60,
            ataque: 10,
            ataque_especial: None,
            defesa: 5,
            inventario: Vec::new(),
        }
    }
}