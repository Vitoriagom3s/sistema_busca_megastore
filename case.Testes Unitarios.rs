use std::collections::HashMap;
use busca::buscar_produto;
use indexacao::indexar_produtos;

#[test]
fn test_buscar_produto() {
    let catalogo = indexar_produtos();
    let resultado = buscar_produto(&catalogo, "eletrônico");
    
    assert_eq!(resultado.len(), 3);
    assert!(resultado.contains(&"Smartphone".to_string()));
    assert!(resultado.contains(&"Laptop".to_string()));
    assert!(resultado.contains(&"Fone de Ouvido".to_string()));
    
    let resultado_vestuarios = buscar_produto(&catalogo, "Calça");
    assert_eq!(resultado_vestuarios.len(), 1);
    assert!(resultado_vestuarios.contains(&"Calça".to_string()));
}