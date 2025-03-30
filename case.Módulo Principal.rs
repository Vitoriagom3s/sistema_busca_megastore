mod busca;
mod indexacao;

fn main() {
    let catalogo = indexacao::indexar_produtos();
    loop {
        println!("Digite um termo para buscar produtos ou 'sair' para encerrar:");
        let mut termo = String::new();
        std::io::stdin().read_line(&mut termo).expect("Erro ao ler a entrada");
        let termo = termo.trim();

        if termo.eq_ignore_ascii_case("sair") {
            break;
        }

        let resultado = busca::buscar_produto(&catalogo, termo);
        if resultado.is_empty() {
            println!("Nenhum produto encontrado para o termo: {}", termo);
        } else {
            for produto in resultado {
                println!("Produto encontrado: {}", produto);
            }
        }
    }
}