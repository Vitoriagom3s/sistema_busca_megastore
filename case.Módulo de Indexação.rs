use std::collections::HashMap;

pub fn indexar_produtos() -> HashMap<String, Vec<String>> {
    let mut catalogo: HashMap<String, Vec<String>> = HashMap::new();
    
    // Exemplo de produtos
    catalogo.insert("eletrônico".to_string(), vec!["Smartphone".to_string(), "Laptop".to_string(), "Fone de Ouvido".to_string()]);
    catalogo.insert("vestuário".to_string(), vec!["Camisa".to_string(), "Calça".to_string(), "Jaqueta".to_string()]);
    
    catalogo
}