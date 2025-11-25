pub struct Item
{
    pub nome: String,
    pub descrição: String,
    pub efeito: u32,
}

impl Item 
{
    pub fn gerar_poc_ataque() -> Self
    {
        Self 
        { 
            nome: "Poção_de_ataque".to_string(), 
            descrição: "Aumenta o ataque em 10".to_string(), 
            efeito: 10,
        }
    } 
    pub fn gerar_poc_vida() -> Self
    {
        Self 
        { 
            nome: "Poção_de_vida".to_string(), 
            descrição: "Recupera 30 de vida".to_string(), 
            efeito: 30,
        }
    } 
    pub fn gerar_poc_defesa() -> Self
    {
        Self 
        { 
            nome: "Poção_de_defesa".to_string(), 
            descrição: "Aumenta a defesa em 5".to_string(), 
            efeito: 5, 
        }
    }   
}