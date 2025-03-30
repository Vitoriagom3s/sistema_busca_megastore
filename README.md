# Sistema de Busca Otimizado para Catálogo de Produtos - 
MegaStore
atividade faculdade

# Estrutura do Código

Módulo Principal (src/main.rs):
Inicializa o sistema e executa a busca.

Módulo de Indexação (src/indexacao.rs):
Responsável por indexar os produtos em uma estrutura de dados eficiente, como HashMap.

Módulo de Busca (src/busca.rs):
Implementa a lógica de busca, permitindo que os usuários encontrem produtos com base em termos de pesquisa.

Testes Unitários (tests/busca_tests.rs):
Garante que a funcionalidade de busca opere corretamente.

# Compilação e Execução
Para compilar e executar o sistema de busca, utilize os seguintes comandos no terminal:
cargo build
cargo run

# Execução dos Testes
Para executar os testes unitários, utilize o comando:
cargo test

# Funcionalidades Adicionais
-Busca por Categoria: O sistema agora permite que os usuários busquem produtos não apenas pelo nome, mas também pela categoria.
-Interface Interativa: O loop no main.rs permite que os usuários façam várias buscas até decidirem sair.
-Mensagens de Feedback: O sistema informa ao usuário se nenhum produto foi encontrado para o termo de busca.

# Considerações Finais
-Desempenho: A busca é otimizada para retornar resultados rapidamente, mesmo com um número crescente de produtos.
-Escalabilidade: O sistema é modular e pode ser facilmente expandido para incluir mais funcionalidades, como integração com bancos de dados ou uma interface gráfica.
-Essa implementação fornece uma base robusta para o sistema de busca do MegaStore, permitindo que você adicione mais recursos conforme o necessário! 

