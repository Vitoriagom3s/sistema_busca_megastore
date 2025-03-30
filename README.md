# Sistema de Busca Otimizado para Catálogo de Produtos MegaStore


![image](https://github.com/user-attachments/assets/a322b181-d368-4d74-8ea9-148c25270ab1)


# Sobre o projeto
Este projeto implementa um sistema de busca otimizado para o catálogo de produtos da MegaStore, uma gigante do varejo online. O objetivo é permitir que os clientes encontrem produtos de forma rápida e eficiente, melhorando a experiência de compra. O sistema utiliza a linguagem de programação Rust e estruturas de dados eficientes, como `HashMap`, para indexação e busca.

# Tecnologias Utilizadas
- **Rust**: Linguagem de programação utilizada para implementar o sistema.
- **HashMap**: Estrutura de dados utilizada para indexação e busca de produtos.
- **Cargo**: Gerenciador de pacotes e build system do Rust.

# Funcionalidades
- **Busca por Nome e Categoria**: Permite que os usuários busquem produtos tanto pelo nome quanto pela categoria.
- **Interface Interativa**: O sistema oferece uma interface de linha de comando onde os usuários podem realizar buscas repetidamente.
- **Mensagens de Feedback**: Informa ao usuário se nenhum produto foi encontrado para o termo de busca.

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

**Realize buscas**: Digite um termo para buscar produtos ou digite 'sair' para encerrar.

# Execução dos Testes
Para executar os testes unitários, utilize o comando:
cargo test

-Os testes garantem que a funcionalidade de busca opere corretamente e que os resultados sejam precisos.

# Exemplos de Uso
Após executar o sistema, você pode buscar produtos utilizando termos como:

"eletrônico"
"vestuário"
"Calça"
O sistema retornará uma lista de produtos correspondentes.

# Arquitetura do Sistema
O sistema é dividido em dois módulos principais:

**indexacao**: Responsável por indexar os produtos em um HashMap.

**busca**: Implementa a lógica de busca, permitindo que os usuários encontrem produtos com base em termos de pesquisa.

# Algoritmos e Estruturas de Dados Utilizados
Utilizamos HashMap para indexar produtos e realizar buscas rápidas. A indexação permite que o sistema acesse produtos de forma eficiente, mesmo em um catálogo extenso.

# Funcionalidades Adicionais
**Interface Interativa**: O loop no main.rs permite que os usuários façam várias buscas até decidirem sair.

**Mensagens de Feedback**: O sistema informa ao usuário se nenhum produto foi encontrado para o termo de busca.

# Considerações Finais
**Desempenho**: A busca é otimizada para retornar resultados rapidamente, mesmo com um número crescente de produtos.

**Escalabilidade**: O sistema é modular e pode ser facilmente expandido para incluir mais funcionalidades, como integração com bancos de dados ou uma interface gráfica.

-Sendo assim o sistema foi projetado para ser escalável, permitindo a adição de novos produtos sem comprometer a performance. Testes de desempenho demonstraram que o tempo de resposta é aceitável mesmo com um grande volume de dados.

# Contato
Para dúvidas ou sugestões, entre em contato:

viviamorimgomes08@gmail.com




