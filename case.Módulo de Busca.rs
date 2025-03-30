use std::collections::HashMap;

pub fn buscar_produto(catalogo: &HashMap<String, Vec<String>>, termo: &str) -> Vec<String> {
    let mut resultados = Vec::new();
    
    for (categoria, produtos) in catalogo {
        if categoria.contains(termo) {
            resultados.extend(produtos.clone());
        } else {
            for produto in produtos {
                if produto.contains(termo) {
                    resultados.push(produto.clone());
                }
            }
        }
    }
    
    resultados
}